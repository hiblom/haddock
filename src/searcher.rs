extern crate rand;

use std::sync::mpsc::Receiver;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::searchtype::SearchType;
use crate::searchcommand::SearchCommand;
use crate::position::Position;
use crate::generator;
use crate::move_::Move_;
use crate::tree::Tree;
use crate::evaluation;
use crate::outcome::Outcome;
use crate::global;

pub struct Searcher {
    receiver: Receiver<SearchCommand>,
    base_position: Position,
    search_type: Option<SearchType>,
    start_time: Option<SystemTime>,
    end_time: Option<SystemTime>,
    stop_signal: Arc<AtomicBool>
}

impl Searcher {
    pub fn new(receiver: Receiver<SearchCommand>, base_position: Position, stop_signal: Arc<AtomicBool>) -> Searcher {
        Searcher {
            receiver: receiver,
            base_position: base_position,
            search_type: None,
            start_time: None,
            end_time: None,
            stop_signal: stop_signal
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
            SearchCommand::Quit => {
                return false
            },
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
            _ => ()
        }

        let best_move = self.search_tree(max_depth);
        println!("bestmove {}", Move_::get_fen(best_move));
     }

    fn search_tree(&mut self, max_depth: u64) -> u32 {
        let current_pos = &self.base_position.clone();

        self.set_times();

        let outcome = evaluation::evaluate(&current_pos);
        let mut tree = Tree { 
            start_score: outcome,
            best_score: None, 
            best_move: None,
            sub_trees: None};
        
        let current_node = &mut tree;

        let mut best_move:Option<u32> = None;
        for depth in 1 .. (max_depth + 1) {
            let (stopped, _, _, total_nodes, pv) = self.traverse_tree(current_node, current_pos);
            if stopped {
                break;
            }

            best_move = current_node.best_move;
            //println!("best move so far {}", Move_::get_fen(best_move.unwrap()));
            let score = current_node.best_score.unwrap().to_uci_score(current_pos.active_color);
            let time = self.get_time_elapsed_ms();
            
            let mut nps = 0;
            if time > 0 {
                nps = (total_nodes as u64) * 1000 / time;
            }

            let pv_string = Searcher::get_moves_string(&pv);

            println!("info depth {} score {} time {} nodes {} nps {} pv {}", depth, score, time, total_nodes, nps, pv_string);

            if current_node.best_score.unwrap().end() {
                break;
            }
        }

        return best_move.unwrap(); //some move HAS to be found...
    }

    fn set_times(&mut self) {
        self.start_time = Some(SystemTime::now());

        let mut turn_duration = 0;
        match self.search_type {
            Some(SearchType::CTime(wtime, btime, _, _)) => {
                if self.base_position.active_color == global::COLOR_WHITE && wtime > 0 {
                    turn_duration = self.get_turn_duration(wtime);
                }
                else if self.base_position.active_color == global::COLOR_BLACK && btime > 0 {
                    turn_duration = self.get_turn_duration(btime);
                }
            },
            Some(SearchType::MoveTime(move_time)) => {
                turn_duration = move_time;
            },
            _ => ()
        }

        if turn_duration > 0 {
            self.end_time = Some(self.start_time.unwrap() + Duration::from_millis(turn_duration as u64));
        }
        else {
            self.end_time = None;
        }
    }

    fn get_time_elapsed_ms(&self) -> u64 {
        match self.start_time {
            Some(t) => {
                let dur = SystemTime::now().duration_since(t).expect("SystemTime::duration_since failed");
                1000 * dur.as_secs() + (dur.subsec_millis() as u64)
            }
            None => 0
        }
    }

    fn get_turn_duration(&self, total_time_left: u64) -> u64 {
        //TODO better time management
        //assume game is 100 turns
        if self.base_position.fullmovenumber < 90 {
            let turn_duration = total_time_left / (100 - self.base_position.fullmovenumber as u64);
            println!("turn duration set to {} ms", turn_duration);
            turn_duration
        }
        else {
            //i don't know, how about 1000 ms
            1000
        }
    }

    fn must_stop(&self) -> bool {
        if self.stop_signal.load(Ordering::Relaxed) {
            return true;
        }

        match self.end_time {
            Some(t) => if SystemTime::now() > t {
                true
            } 
            else {
                false
            },
            None => false
        }
    }

    fn traverse_tree(&self, node: &mut Tree, position: &Position) -> (bool, Option<Outcome>, Option<u32>, u32, Vec<u32>) {
        if self.must_stop() {
            return (true, None, None, 0, Vec::new());
        }

        let mut current_nodes = 0u32;
        let mut pv: Vec<u32> = Vec::new();
        match node.best_score {
            Some(mut s) => {
                //go back when we reached an end (mate, draw)
                if s.end() {
                    //increase mate count
                    if position.active_color == global::COLOR_BLACK {
                        match s {
                            Outcome::WhiteIsMate(n) => s = Outcome::WhiteIsMate(n + 1),
                            Outcome::BlackIsMate(n) => s = Outcome::BlackIsMate(n + 1),
                            _ => ()
                        }
                    }
                    return (false, Some(s), node.best_move, current_nodes,  Vec::new())
                }
                //we must go deeper
                node.best_score = None;
                node.best_move = None;
                let mut best_sub_pv: Vec<u32> = Vec::new();
                if let Some(mut sub_trees) = node.sub_trees.take() {
                    for (move_, sub_tree) in sub_trees.iter_mut() {
                        let mut pos = position.clone();
                        pos.apply_move(*move_);
                        let (stop, sub_best_score, _, sub_nodes, sub_pv) = self.traverse_tree(sub_tree, &pos);
                        if stop {
                            return (true, None, None, 0, Vec::new());
                        }
                        current_nodes += sub_nodes;

                        if Searcher::is_better_outcome(&sub_best_score, &node.best_score, position.active_color) {
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
                let mut sub_trees: HashMap<u32, Tree> = HashMap::new();
                let legal_moves = generator::generate_legal_moves(position);
                
                for legal_move in &legal_moves {
                    let mut new_pos = position.clone();
                    new_pos.apply_move(*legal_move);
                    let outcome = evaluation::evaluate(&new_pos);
                    sub_trees.insert(*legal_move, Tree { 
                        start_score: outcome, 
                        best_score: None, 
                        best_move: None, 
                        sub_trees: None });

                    if Searcher::is_better_outcome(&Some(outcome), &node.best_score, position.active_color) {
                        node.best_score = Some(outcome);
                        node.best_move = Some(*legal_move);
                    }
                }
                current_nodes += legal_moves.len() as u32;
                node.sub_trees = Some(sub_trees);
                match node.best_move {
                    Some(m) => pv.push(m),
                    None => ()
                }
            }
        }
        (false, node.best_score, node.best_move, current_nodes, pv)
    }

    fn is_better_outcome(score: &Option<Outcome>, current_best_score: &Option<Outcome>, active_color: u8) -> bool {
        if current_best_score.is_none() {
            return true;
        }

        if score.is_none() {
            return false;
        }

        if active_color == global::COLOR_WHITE {
            return score.unwrap() > current_best_score.unwrap();
        }
        else {
            return score.unwrap() < current_best_score.unwrap();
        }
    }

    fn get_moves_string(moves: &Vec<u32>) -> String {
        let mut moves_string = "".to_string();
        for mv in moves {
            if moves_string.len() > 0 {
                moves_string.push_str(" ");
            }
            moves_string.push_str(&Move_::get_fen(*mv));
        }
        moves_string
    }
}

