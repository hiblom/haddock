extern crate rand;

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use crate::evaluation;
use crate::generator::Generator;
use crate::global;
use crate::move_::Move_;
use crate::outcome::Outcome;
use crate::position::Position;
use crate::searchcommand::SearchCommand;
use crate::searchtype::SearchType;
use crate::tree::Tree;

pub struct Searcher {
    receiver: Receiver<SearchCommand>,
    base_position: Position,
    search_type: Option<SearchType>,
    start_time: Option<SystemTime>,
    end_time: Option<SystemTime>,
    stop_signal: Arc<AtomicBool>,
    node_count: u32
}

impl Searcher {
    pub fn new(receiver: Receiver<SearchCommand>, base_position: Position, stop_signal: Arc<AtomicBool>) -> Searcher {
        Searcher {
            receiver: receiver,
            base_position: base_position,
            search_type: None,
            start_time: None,
            end_time: None,
            stop_signal: stop_signal,
            node_count: 0
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

        struct StackState {
            position: Position,
            moves: Vec<Move_>,
            current_index: usize,
            score: Option<Outcome>,
            sub_pv: Option<Vec<Move_>>
        }

        let mut stack: Vec<StackState> = Vec::new();

        stack.push( StackState { 
            position: current_pos, 
            moves: Generator::new(&current_pos).generate_legal_moves(),
            current_index: 0,
            score: None,
            sub_pv: None });

        let mut best_move: Option<Move_> = None;
        
        //vector of hashmap to keep the best moves and refutes in, starting at level 1
        let mut best_moves: Vec<HashMap<Move_, Move_>> = vec![
            HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(),
            HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new()];

        for max_iter_depth in 1..(max_depth + 1) as usize {
            //println!("depth iteration {}", max_iter_depth);

            let mut d: usize = 1;
            stack[0].current_index = 0;
            stack[0].score = None;
            stack[0].sub_pv = None;

            //put best move at top, for alpha/beta
            match best_move {
                Some(bm) => {
                    let i_ = stack[0].moves.iter().position(|&m| m == bm);
                    match i_ {
                        Some(i) => {
                            stack[0].moves.remove(i);
                            stack[0].moves.insert(0, bm)
                        },
                        None => () //??
                    }
                },
                None => ()
            }

            while !self.must_stop() {
                //alternatives:
                //make new node, increase depth
                //evaluate, decrease depth
                //next move at current depth, increase depth
                //out of moves, destroy node, decrease depth
                
                if d == max_iter_depth {
                    //evaluate
                    let i = d - 1;
                    let mut pos = stack[i].position.clone();
                    let color = pos.get_active_color();

                    let move_ = stack[i].moves[stack[i].current_index];
                    //println!("evaluating {} at depth {}", move_.get_fen(), d);
                    pos.apply_move(move_);

                    if pos.was_capture() {
                        //println!("evaluating capture exchange after move {} on pos\n{}", move_.to_fen(), pos);
                        let (_, square_to) = move_.get_squares();
                        let gen = Generator::new(&pos);
                        pos = gen.capture_exchange(square_to);
                        //println!("pos after capture exchange:\n{}", pos);
                    }

                    let score = Some(evaluation::evaluate(&pos, d as i32, false));
                    self.node_count += 1;

                    if Searcher::is_better_outcome(&score, &stack[i].score, color) {
                        stack[i].score = score;
                        stack[i].sub_pv = Some(vec![move_]);

                        if d >= 2 && d <= 11 {
                            best_moves[d - 2].insert(stack[d - 2].moves[stack[d - 2].current_index], move_);
                        }

                        //look for alpha-beta cutoff opportunities
                        //compare score with score 2 levels back
                        //if score is worse, move at level d - 2 is refuted
                        if d > 1 {
                            if Searcher::is_better_outcome(&stack[d - 2].score, &score, 1 - color) {
                                stack.pop();
                                d -= 2;
                                continue;
                            }
                        }
                    }
                    d -= 1;

                } else {
                    if stack.len() < d + 1 {
                        //create new node
                        let current_pos = stack[d - 1].position;
                        let mut pos = current_pos.clone();
                        let move_ = stack[d - 1].moves[stack[d - 1].current_index];
                        pos.apply_move(move_);
                        

                        let mut moves = Generator::new(&pos).generate_legal_moves();

                        //sort: best move and refutes from previous iteration go to the top
                        if d >= 1 && d <= 10 {
                            if best_moves[d - 1].contains_key(&move_) {
                                let p_ = moves.iter().position(|&m| m == best_moves[d - 1][&move_]);
                                match p_ {
                                    Some(p) => {
                                        //println!("Putting move {} -> {} at the top", move_.get_fen(), best_moves[&move_].get_fen());
                                        moves.remove(p);
                                        moves.insert(0, best_moves[d - 1][&move_]);
                                    },
                                    None => ()
                                }
                            }
                        }

                        if moves.len() == 0 {
                            //check mate or stale mate
                            let color = pos.get_active_color();
                            let score: Option<Outcome>;
                            if Generator::new(&pos).is_check(color) {
                                if color == global::COLOR_WHITE {
                                    score = Some(Outcome::WhiteIsMate(d as i32));
                                } else {
                                    score = Some(Outcome::BlackIsMate(d as i32));
                                }
                            } else {
                                score = Some(Outcome::DrawByStalemate(d as i32));
                            }
                            if Searcher::is_better_outcome(&score, &stack[d - 1].score, 1 - color) {
                                stack[d - 1].score = score;
                                stack[d - 1].sub_pv = Some(vec![move_])
                            }
                            d -= 1;
                        } else {
                            stack.push( StackState { 
                                position: pos, 
                                moves: moves,
                                current_index: 0,
                                score: None,
                                sub_pv: None });
                            d += 1
                        }
                    }
                    else {
                        //println!("index {}/{}, depth {}", stack[d].current_index, stack[d].moves.len(), d);
                        let new_index = stack[d].current_index + 1;
                        if new_index < stack[d].moves.len() {
                            //next move
                            stack[d].current_index = new_index;
                            d += 1;
                        } else {
                            //go back a level
                            if d == 0 {
                                break;
                            }

                            //update parent best score
                            if Searcher::is_better_outcome(&stack[d].score, &stack[d - 1].score, stack[d - 1].position.get_active_color()) {
                                stack[d - 1].score = stack[d].score;
                                if let Some(mut child_v) = stack[d].sub_pv.take() {
                                    let mut parent_v = Vec::new();
                                    let parent_move = stack[d - 1].moves[stack[d - 1].current_index];
                                    let cv_clone = child_v.clone();
                                    parent_v.push(parent_move);
                                    parent_v.append(&mut child_v);
                                    stack[d - 1].sub_pv = Some(parent_v);

                                    if (d >= 1 && d <= 10) && cv_clone.len() > 0 {
                                        //println!("update best moves {} -> {}", parent_move.get_fen(), cv_clone[0].get_fen());
                                        best_moves[d - 1].insert(parent_move, cv_clone[0]);
                                    }

                                    //look for alpha beta cutoff opportunities
                                    if d > 1 {
                                        if Searcher::is_better_outcome(&stack[d - 2].score, &stack[d].score, stack[d - 2].position.get_active_color()) {
                                            stack.pop();
                                            stack.pop();
                                            d -= 2;
                                            continue;
                                        }
                                    }
                                }
                            }

                            stack.pop();
                            d -= 1;
                        }
                    }
                }
            }

            //let mut hashmap_length = 0;
            //for hashmap_depth in 0..best_moves.len() {
            //    hashmap_length += best_moves[hashmap_depth].len();
            //}
            //println!("hashmap length: {:?}", hashmap_length);

            if self.must_stop() {
                break;
            }

            if stack[0].score.is_none() || stack[0].sub_pv.is_none() {
                break;
            }

            let time = self.get_time_elapsed_ms();
            let mut nps = self.node_count as u64;
            if time > 0 {
                nps = nps * 1000 / time;
            }

            let uci_score = stack[0].score.unwrap().to_uci_score(current_pos.get_active_color());
            let pv = stack[0].sub_pv.take().unwrap();
            let pv_string = Searcher::get_moves_string(&pv);

            println!(
                "info depth {} score {} time {} nodes {} nps {} pv {}",
                max_iter_depth, uci_score, time, self.node_count, nps, pv_string
            );
            best_move = Some(pv[0]);
            if stack[0].score.unwrap().end() {
                break;
            }
        }

        match best_move {
            Some(m) => return m,
            None => panic!("Best move not found!")
        }

    }

    #[allow(dead_code)]
    fn search_tree(&mut self, max_depth: u64) -> Move_ {
        let current_pos = &self.base_position.clone();

        self.node_count = 0;
        self.set_times();

        let mut tree = Tree {
            best_score: None,
            best_move: None,
            sub_trees: None,
        };

        let current_node = &mut tree;

        let mut best_move: Option<Move_> = None;
        for depth in 1..(max_depth + 1) {
            let (stopped, _, _, pv) = self.traverse_tree(current_node, current_pos, 1);
            if stopped {
                break;
            }

            best_move = current_node.best_move;
            let best_score;
            match current_node.best_score {
                Some(s) => best_score = s,
                None => panic!("No best score found in node"),
            }

            let score = best_score.to_uci_score(current_pos.get_active_color());
            let time = self.get_time_elapsed_ms();

            let mut nps = 0;
            if time > 0 {
                nps = (self.node_count as u64) * 1000 / time;
            }

            let pv_string = Searcher::get_moves_string(&pv);

            println!(
                "info depth {} score {} time {} nodes {} nps {} pv {}",
                depth, score, time, self.node_count, nps, pv_string
            );

            match current_node.best_score {
                Some(s) => {
                    if s.end() {
                        break;
                    }
                }
                None => panic!("No best score found in node!"),
            }
        }

        match best_move {
            Some(m) => {
                return m;
            }
            None => panic!("Best move not found!"),
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

    fn traverse_tree(&mut self, node: &mut Tree, position: &Position, depth: i32) -> 
        (bool, Option<Outcome>, Option<Move_>, Vec<Move_>) {
        if self.must_stop() {
            return (true, None, None, Vec::new());
        }

        let mut pv: Vec<Move_> = Vec::new();
        match node.best_score {
            Some(s) => {
                //go back when we reached an end (mate, draw)
                if s.end() {
                    return (false, Some(s), node.best_move, Vec::new())
                }
                //we must go deeper
                node.best_score = None;
                node.best_move = None;
                let mut best_sub_pv: Vec<Move_> = Vec::new();
                if let Some(mut sub_trees) = node.sub_trees.take() {
                    for (move_, sub_tree) in sub_trees.iter_mut() {
                        let mut pos = position.clone();
                        pos.apply_move(*move_);
                        let (stop, sub_best_score, _, sub_pv) = self.traverse_tree(sub_tree, &pos, depth + 1);
                        if stop {
                            return (true, None, None, Vec::new());
                        }

                        if Searcher::is_better_outcome(&sub_best_score, &node.best_score, position.get_active_color()) {
                            node.best_score = sub_best_score;
                            node.best_move = Some(*move_);
                            best_sub_pv = sub_pv;
                        }

                    }
                    node.sub_trees = Some(sub_trees);
                    match node.best_move {
                        Some(m) => pv.push(m),
                        None => ()
                    }
                    pv.append(&mut best_sub_pv);
                }
            },
            None => {
                let mut sub_trees: HashMap<Move_, Tree> = HashMap::new();
                let moves = Generator::new(position).generate_moves();
                
                for move_ in &moves {
                    let mut new_pos = position.clone();
                    new_pos.apply_move(*move_);
                    let outcome = evaluation::evaluate(&new_pos, depth, true);

                    match outcome {
                        Outcome::Illegal(_) => {
                            continue;
                        },
                        _ => ()
                    }

                    sub_trees.insert(*move_, Tree { 
                        best_score: None, 
                        best_move: None, 
                        sub_trees: None });

                    if Searcher::is_better_outcome(&Some(outcome), &node.best_score, position.get_active_color()) {
                        node.best_score = Some(outcome);
                        node.best_move = Some(*move_);
                    }
                }

                if sub_trees.len() == 0 {
                    //check mate or stale mate
                    if Generator::new(position).is_check(position.get_active_color()) {
                        if position.get_active_color() == global::COLOR_WHITE {
                            node.best_score = Some(Outcome::WhiteIsMate(depth));
                        }
                        else {
                            node.best_score = Some(Outcome::BlackIsMate(depth));
                        }
                    }
                    else {
                        node.best_score = Some(Outcome::DrawByStalemate(depth));
                    }
                }

                self.node_count += moves.len() as u32;
                node.sub_trees = Some(sub_trees);
                match node.best_move {
                    Some(m) => pv.push(m),
                    None => ()
                }
            }
        }
        (false, node.best_score, node.best_move, pv)
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
}
