extern crate rand;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::global;
use crate::evaluation;
use crate::generator::Generator;
use crate::move_::Move_;
use crate::outcome::Outcome;
use crate::position::Position;
use crate::searchcommand::SearchCommand;
use crate::searchtype::SearchType;
use crate::moveresult::MoveResult;
use crate::hash_counter::HashCounter;
use crate::transposition_table::TranspositionTable;

pub struct Searcher {
    receiver: Receiver<SearchCommand>,
    base_position: Position,
    search_type: Option<SearchType>,
    start_time: Option<SystemTime>,
    end_time: Option<SystemTime>,
    stop_signal: Arc<AtomicBool>,
    node_count: u32,
    transposition_count: u32,
    history: HashCounter,
    transposition_table: TranspositionTable
}

struct RecursiveSearchRequest<'a> {
    position: &'a Position,
    depth: usize,
    horizon: usize,
    alphabeta: Option<Outcome>
}

struct RecursiveSearchResponse {
    score: Outcome,
    variant: Vec<Move_>
}


impl Searcher {
    pub fn new(receiver: Receiver<SearchCommand>, base_position: Position, stop_signal: Arc<AtomicBool>, history: HashCounter) -> Searcher {
        Searcher {
            receiver,
            base_position,
            search_type: None,
            start_time: None,
            end_time: None,
            stop_signal,
            node_count: 0,
            transposition_count: 0,
            history,
            transposition_table: TranspositionTable::new()
        }
    }

    pub fn start(&mut self) {
        loop {
            let command = self.receiver.recv().unwrap();
            if !self.handle_command(&command) {
                break;
            }
        }
    }

    fn handle_command(&mut self, command: &SearchCommand) -> bool {
        match command {
            SearchCommand::Quit => return false,
            SearchCommand::FindBestMove(search_type) => {
                self.search_type = Some(*search_type);
                self.handle_command_find_best_move();
            }
        }
        true
    }

    fn handle_command_find_best_move(&mut self) {
        let mut max_depth = 1000;
        match self.search_type {
            Some(SearchType::Depth(n)) => max_depth = n,
            _ => (),
        }

        let best_move = self.search_tree_3(max_depth);
        println!("bestmove {}", best_move.to_fen());
    }

    fn search_tree_3(&mut self, max_depth: u64) -> Move_ {

        self.node_count = 0;
        self.set_times();
        let current_pos = self.base_position.clone();

        let mut best_move: Option<Move_> = None;

        for max_iter_depth in 1..(max_depth + 1) as usize {

            let request = RecursiveSearchRequest {
                position: &current_pos,
                depth: 0,
                horizon: max_iter_depth,
                alphabeta: None
            };

            let response_ = self.recursive_search(request);

            //println!("history table size: {}", self.history.get_len());
            println!("transposition table size: {}, hits: {}", self.transposition_table.len(), self.transposition_count);

            if self.must_stop() {
                break;
            }

            if let Some(response) = response_ {
                let time = self.get_time_elapsed_ms();
                let mut nps = self.node_count as u64;
                if time > 0 {
                    nps = nps * 1000 / time;
                }

                let uci_score = response.score.to_uci_score(current_pos.get_active_color());
                let pv_string = Searcher::get_moves_string(&response.variant);

                println!(
                    "info depth {} score {} time {} nodes {} nps {} pv {}",
                    max_iter_depth, uci_score, time, self.node_count, nps, pv_string
                );
                best_move = Some(response.variant[0]);
                if response.score.end() {
                    break;
                }

                //check time, if we don't have enough time for next iteration, stop
                match self.get_time_left() {
                    None => (),
                    Some(duration) => {
                        if time * 2 > duration {
                            break;
                        }
                    }
                }
            }

        }

        match best_move {
            Some(m) => return m,
            None => panic!("Best move not found!")
        }
    }

    fn recursive_search(&mut self, request: RecursiveSearchRequest) -> Option<RecursiveSearchResponse> {
        if self.must_stop() {
            return None;
        }
        self.node_count += 1;

        if request.horizon == 0 {
            return Some(RecursiveSearchResponse {
                score: evaluation::evaluate(&request.position, request.horizon as i32),
                variant: Vec::new()
            });
        }

        //check transposition table
        if let Some((mv, outcome)) = self.transposition_table.get(request.position.get_hash(), request.horizon as i32) {
            self.transposition_count += 1;
            return Some(RecursiveSearchResponse {
                score: outcome,
                variant: vec![mv]
            });
        }

        let active_color = request.position.get_active_color();
        let other_color = 1 - active_color;
        let generator = Generator::new(&request.position);

        let mut current_best_score: Option<Outcome> = None;
        let mut current_best_variant: Vec<Move_> = Vec::new();

        //TODO examine "null move" -> dangerous in end game / zugzwang positions
        if request.depth > 3 && request.horizon == 1 {
            current_best_score = Some(evaluation::evaluate(&request.position, request.horizon as i32));

            //aplphabeta cutoff
            if Searcher::is_better_outcome(&request.alphabeta, &current_best_score, other_color) {
                return Some(RecursiveSearchResponse {
                    score : current_best_score.unwrap(),
                    variant: current_best_variant
                });
            }
        }

        let mut moves = generator.generate_moves();

        //sort moves; killer move first
        if let Some(mv) = self.transposition_table.get_best_move(request.position.get_hash()) {
            if let Some(p) = moves.iter().position(|&m| m == mv) {
                moves.remove(p);
                moves.insert(0, mv);
            }
        }

        for mv in moves {
            if request.depth > 3 && request.horizon == 1 && !(mv.is_capture() || mv.is_promotion()) {
                //do not examine silent moves after depth 3
                continue;
            }

            let score: Option<Outcome>;
            let mut variant: Vec<Move_> = Vec::new();

            match generator.try_apply_move(mv, &self.history) {
                MoveResult::Next(mut child_pos) => {
                    //SEE
                    if request.horizon == 1 && mv.is_capture() {
                        let (_, square_to) = mv.get_squares();
                        child_pos = Generator::new(&child_pos).capture_exchange(square_to);
                    }
                    let child_request = RecursiveSearchRequest {
                        position: &child_pos,
                        depth: request.depth + 1,
                        horizon: request.horizon - 1,
                        alphabeta: current_best_score
                    };
                    self.history.incr(child_pos.get_hash());
                    if let Some(child_response) = self.recursive_search(child_request) {
                        self.history.decr(child_pos.get_hash());
                        score = Some(child_response.score);
                        variant = child_response.variant;
                    } else {
                        self.history.decr(child_pos.get_hash());
                        return None;
                    }
                },
                MoveResult::Illegal => continue,
                MoveResult::Draw => score = Some(Outcome::Draw(request.horizon as i32))
            }

            if Searcher::is_better_outcome(&score, &current_best_score, active_color) {
                current_best_score = score;
                current_best_variant = vec![mv];
                current_best_variant.append(&mut variant);

                self.transposition_table.insert(
                    request.position.get_hash(),
                    request.horizon as i32,
                    Some(mv),
                    score
                );

                //alphabeta cutoff
                if Searcher::is_better_outcome(&request.alphabeta, &score, other_color) {
                    return Some(RecursiveSearchResponse {
                        score : score.unwrap(),
                        variant: current_best_variant
                    });
                }
            }
        }

        //no score means mate or stalemate
        if current_best_score.is_none() {
            //check mate or stale mate
            if generator.is_check(active_color) {
                if active_color == global::COLOR_WHITE {
                    current_best_score = Some(Outcome::WhiteIsMate(request.horizon as i32));
                } else {
                    current_best_score = Some(Outcome::BlackIsMate(request.horizon as i32));
                }
            } else {
                current_best_score = Some(Outcome::Draw(request.horizon as i32));
            }
        }

        return Some(RecursiveSearchResponse {
            score : current_best_score.unwrap(),
            variant: current_best_variant
        });
    }

    fn set_times(&mut self) {
        self.start_time = Some(SystemTime::now());

        let mut turn_duration = 0;
        match self.search_type {
            Some(SearchType::CTime(wtime, btime, _, _, movestogo)) => {
                if self.base_position.get_active_color() == global::COLOR_WHITE && wtime > 0 {
                    turn_duration = self.get_turn_duration(wtime, movestogo);
                } else if self.base_position.get_active_color() == global::COLOR_BLACK && btime > 0
                {
                    turn_duration = self.get_turn_duration(btime, movestogo);
                }
            }
            Some(SearchType::MoveTime(move_time)) => {
                turn_duration = move_time;
            }
            _ => (),
        }

        if turn_duration > 0 {
            self.end_time =
                Some(self.start_time.unwrap() + Duration::from_millis(turn_duration as u64));
        } else {
            self.end_time = None;
        }
    }

    fn get_time_elapsed_ms(&self) -> u64 {
        match self.start_time {
            Some(t) => {
                let dur = SystemTime::now()
                    .duration_since(t)
                    .expect("SystemTime::duration_since failed");
                1000 * dur.as_secs() + (dur.subsec_millis() as u64)
            }
            None => 0,
        }
    }

    fn get_turn_duration(&self, total_time_left: u64, movestogo: u64) -> u64 {
        let result;
        if movestogo > 0 {
            result = (total_time_left / movestogo) * 99 / 100; //to be on the safe side, take 99%
        } else {
            if self.base_position.get_fullmovenumber() < 40 {
                result = total_time_left / (50 - self.base_position.get_fullmovenumber() as u64);
            } else {
                //from move 40 onward, we have less and less time
                result = total_time_left / 20;
            }
        }

        //println!("Haddock will think for {} ms", result);
        result
    }

    fn must_stop(&self) -> bool {
        if self.node_count & 0xFFFFu32 != 0 {
            return false;
        }

        if self.stop_signal.load(Ordering::Relaxed) {
            return true;
        }

        match self.end_time {
            Some(t) => {
                if SystemTime::now() > t {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn get_time_left(&self) -> Option<u64> {
        let now = SystemTime::now();
        match self.end_time {
            Some(et) => {
                if now > et {
                    return Some(0);
                } else {
                    let dur = et.duration_since(now).expect("SystemTime::duration_since failed");
                    return Some(1000 * dur.as_secs() + dur.subsec_millis() as u64);
                }
            }
            None => None,
        }
    }

    fn is_better_outcome(score: &Option<Outcome>, current_best_score: &Option<Outcome>, active_color: u8,) -> bool {
        if score.is_none() {
            return false;
        }

        if current_best_score.is_none() {
            return true;
        }

        if active_color == global::COLOR_WHITE {
            return score.unwrap() > current_best_score.unwrap();
        } else {
            return score.unwrap() < current_best_score.unwrap();
        }
    }

    fn get_moves_string(moves: &Vec<Move_>) -> String {
        let mut moves_string = "".to_string();
        for mv in moves {
            if moves_string.len() > 0 {
                moves_string.push_str(" ");
            }
            moves_string.push_str(&mv.to_fen());
        }
        moves_string
    }
}
