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
use crate::transposition_table::{TranspositionTable, TranspositionTableEntry};

struct PathNode {
    position: Position,
    moves: Vec<Move_>,
    moves_set: bool,
    current_index: usize,
    score: Option<Outcome>,
    sub_pv: Option<Vec<Move_>>
}

pub struct Searcher {
    receiver: Receiver<SearchCommand>,
    base_position: Position,
    search_type: Option<SearchType>,
    start_time: Option<SystemTime>,
    end_time: Option<SystemTime>,
    stop_signal: Arc<AtomicBool>,
    path: Vec<PathNode>,
    node_count: u32,
    history: HashCounter,
    transposition_table: TranspositionTable
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
            path: Vec::new(),
            node_count: 0,
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

        let best_move = self.search_tree_2(max_depth);
        println!("bestmove {}", best_move.to_fen());
    }

    fn search_tree_2(&mut self, max_depth: u64) -> Move_ {

        self.node_count = 0;
        self.set_times();

        let current_pos = self.base_position.clone();

        self.path = Vec::new();

        self.path.push( PathNode { 
            position: current_pos, 
            moves: Vec::new(),
            moves_set: false,
            current_index: 0,
            score: None,
            sub_pv: None });

        let mut best_move: Option<Move_> = None;
        
        self.transposition_table = TranspositionTable::new();

        for max_iter_depth in 0..max_depth as usize {
            let mut d: usize = 0;
            self.path[0].current_index = 0;
            self.path[0].score = None;
            self.path[0].sub_pv = None;

            while !self.must_stop() {
                if !self.path[d].moves_set {
                    self.get_stack_moves(d);
                    if d == 0 {
                        self.sort_first_stack_moves(&best_move);
                    } else {
                        self.sort_stack_moves(d);
                    }
                    if d == max_iter_depth {
                        d = self.evaluate_stack_moves(d);
                    } else {
                        d = self.progress_stack(d);
                    }
                    continue;
                }

                self.path[d].current_index += 1;
                if self.path[d].current_index < self.path[d].moves.len() {
                    d = self.progress_stack(d);
                    continue;
                }

                //(check/stale)mate
                if self.path[d].score.is_none() {
                    //check mate or stale mate
                    let color = self.path[d].position.get_active_color();
                    //let score: Option<Outcome>;
                    if Generator::new(&self.path[d].position).is_check(color) {
                        if color == global::COLOR_WHITE {
                            self.path[d].score = Some(Outcome::WhiteIsMate(d as i32));
                        } else {
                            self.path[d].score = Some(Outcome::BlackIsMate(d as i32));
                        }
                    } else {
                        self.path[d].score = Some(Outcome::Draw(d as i32));
                    }
                }

                //go back a level
                if d == 0 {
                    break;
                }

                //update parent best score
                d = self.regress_stack(d);
            }

            println!("killer move map size: {}", self.transposition_table.len());

            if self.must_stop() {
                break;
            }

            if self.path[0].score.is_none() || self.path[0].sub_pv.is_none() {
                break;
            }

            let time = self.get_time_elapsed_ms();
            let mut nps = self.node_count as u64;
            if time > 0 {
                nps = nps * 1000 / time;
            }

            let uci_score = self.path[0].score.unwrap().to_uci_score(current_pos.get_active_color());
            let pv = self.path[0].sub_pv.take().unwrap();
            let pv_string = Searcher::get_moves_string(&pv);

            println!(
                "info depth {} score {} time {} nodes {} nps {} pv {}",
                max_iter_depth + 1, uci_score, time, self.node_count, nps, pv_string
            );
            best_move = Some(pv[0]);
            if self.path[0].score.unwrap().end() {
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

        self.transposition_table = TranspositionTable::new(); //clean up

        match best_move {
            Some(m) => return m,
            None => panic!("Best move not found!")
        }

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

        println!("Haddock will think for {} ms", result);
        result
    }

    fn must_stop(&self) -> bool {
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
        if current_best_score.is_none() {
            return true;
        }

        if score.is_none() {
            return false;
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

    fn get_stack_moves(&mut self, depth: usize) {
        let pos = self.path[depth].position;
        let generator = Generator::new(&pos);
        let moves = generator.generate_moves();
        self.path[depth].moves = moves;
        self.path[depth].moves_set = true;
    }

    fn evaluate_stack_moves(&mut self, depth: usize) -> usize {
        let pos = self.path[depth].position;
        let color = pos.get_active_color();
        let generator = Generator::new(&pos);

        //examine "null move"
        if depth > 4 {
            let score = Some(evaluation::evaluate(&pos, depth as i32));
            if Searcher::is_better_outcome(&score, &self.path[depth].score, color) {
                let parent_depth = depth - 1;
                self.path[depth].score = score;
                self.path[depth].sub_pv = Some(Vec::new());

                //beta cutoff
                if depth > 0 {
                    if Searcher::is_better_outcome(&self.path[parent_depth].score, &score, 1 - color) {
                        self.history.decr(self.path[parent_depth].position.get_hash());
                        self.path.pop();
                        return parent_depth;
                    }
                }
            }
        }

        let len = self.path[depth].moves.len();
        for i in 0..len {
            let mv = self.path[depth].moves[i];
            if depth > 4 && !(mv.is_capture() || mv.is_promotion()) {
                //do not examine silent moves after depth 4
                continue;
            }
            self.node_count += 1;

            let score;
            match generator.try_apply_move(mv, &self.history) {
                MoveResult::Next(mut child_pos) => {
                    if mv.is_capture() {
                        let (_, square_to) = mv.get_squares();
                        child_pos = Generator::new(&child_pos).capture_exchange(square_to);
                    }
                    score = Some(evaluation::evaluate(&child_pos, depth as i32));
                },
                MoveResult::Illegal => continue,
                MoveResult::Draw => score = Some(Outcome::Draw(depth as i32))
            }

            if Searcher::is_better_outcome(&score, &self.path[depth].score, color) {
                self.path[depth].score = score;
                self.path[depth].sub_pv = Some(vec![mv]);

                //cutoff
                if depth > 0 {
                    let entry = TranspositionTableEntry{ best_move : mv, outcome : None };
                    self.transposition_table.insert(pos.get_hash(), entry);
                    let parent_depth = depth - 1;
                    if Searcher::is_better_outcome(&self.path[parent_depth].score, &score, 1 - color) {
                        self.history.decr(self.path[parent_depth].position.get_hash());
                        self.path.pop();
                        return parent_depth;
                    }
                }
            }
        }

        self.path[depth].current_index = self.path[depth].moves.len() - 1;
        depth
    }

    fn progress_stack(&mut self, depth: usize) -> usize {
        let mut child_pos: Option<Position> = None;

        let len = self.path[depth].moves.len();
        while self.path[depth].current_index < len {
            let mv = self.path[depth].moves[self.path[depth].current_index];
            match Generator::new(&self.path[depth].position).try_apply_move(mv, &self.history) {
                MoveResult::Illegal => {
                    self.path[depth].current_index += 1;
                }
                MoveResult::Draw => {
                    self.history.incr(self.path[depth].position.get_hash());
                    self.path[depth].score = Some(Outcome::Draw(depth as i32));
                    self.path[depth].sub_pv = Some(vec![mv]);
                    break;
                }
                MoveResult::Next(p) => {
                    self.history.incr(self.path[depth].position.get_hash());
                    child_pos = Some(p);
                    break;
                }
            }
        }

        if child_pos.is_none() {
            return depth;
        }

        self.path.push(PathNode {
            position: child_pos.unwrap(),
            moves: Vec::new(),
            moves_set: false,
            current_index: 0,
            score: None,
            sub_pv: None
        });


        depth + 1
    }

    fn regress_stack(&mut self, depth: usize) -> usize {
        let parent_depth = depth - 1;
        if Searcher::is_better_outcome(&self.path[depth].score, &self.path[parent_depth].score, self.path[parent_depth].position.get_active_color()) {
            self.path[parent_depth].score = self.path[depth].score;
            
            let mut parent_v = Vec::new();
            let parent_move = self.path[parent_depth].moves[self.path[parent_depth].current_index];
            parent_v.push(parent_move);
            let mut child_mv: Option<Move_> = None;
            if let Some(mut child_v) = self.path[depth].sub_pv.take() {
                if child_v.len() > 0 {
                    child_mv = Some(child_v[0]);
                }
                parent_v.append(&mut child_v);
            }
            self.path[parent_depth].sub_pv = Some(parent_v);

            if depth >= 1 && child_mv.is_some() {
                let entry = TranspositionTableEntry{ best_move : child_mv.unwrap(), outcome : None };
                self.transposition_table.insert(self.path[parent_depth].position.get_hash(), entry);
            }

            //look for beta cutoff opportunities
            if depth > 1 {
                if Searcher::is_better_outcome(&self.path[depth - 2].score, &self.path[depth].score, self.path[depth - 2].position.get_active_color()) {
                    self.history.decr(self.path[parent_depth].position.get_hash());
                    self.history.decr(self.path[depth - 2].position.get_hash());
                    self.path.pop();
                    self.path.pop();
                    return depth - 2;
                }
            }
        }
        self.history.decr(self.path[parent_depth].position.get_hash());
        self.path.pop();
        depth - 1
    }

    fn sort_first_stack_moves(&mut self, best_move: &Option<Move_>) {
        if let Some(bm) = best_move {
            if let Some(i) = self.path[0].moves.iter().position(|&m| m == *bm) {
                self.path[0].moves.remove(i);
                self.path[0].moves.insert(0, *bm)
            }
        }
    }

    fn sort_stack_moves(&mut self, depth: usize) {
        let hash =  self.path[depth - 1].position.get_hash();

        if let Some(entry) = self.transposition_table.get(hash) {
            if let Some(p) = self.path[depth].moves.iter().position(|&m| m == entry.best_move) {
                self.path[depth].moves.remove(p);
                self.path[depth].moves.insert(0, entry.best_move);
            }
        }
    }
}
