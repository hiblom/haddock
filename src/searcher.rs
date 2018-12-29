extern crate rand;

use std::sync::mpsc::Receiver;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

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
    base_position: Position
}

impl Searcher {
    pub fn new(receiver: Receiver<SearchCommand>, base_position: Position) -> Searcher {
        println!("Initializing searcher");
        Searcher {
            receiver: receiver,
            base_position: base_position
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

    fn handle_command(&self, command: &SearchCommand) -> bool {
        match command {
            SearchCommand::Quit => {
                println!("Shutting down searcher");
                return false
            },
            SearchCommand::FindBestMove => self.handle_command_find_best_move()
        }
        true
    }

    fn handle_command_find_best_move(&self) {
        //let legal_moves = generator::generate_legal_moves(&self.base_position);
        //let mut rng = rand::thread_rng();
        //let i = rng.gen_range(0, legal_moves.len());

        let best_move = self.search_tree(4);

        println!("bestmove {}", Move_::get_fen(best_move));
     }

    fn search_tree(&self, max_depth: u32) -> u32 {
        let current_pos = &self.base_position.clone();

        //start the clock
        let start = SystemTime::now();

        let outcome = evaluation::evaluate(&current_pos);
        let mut tree = Tree { 
            start_score: outcome,
            best_score: None, 
            best_move: None,
            sub_trees: None};
        
        let current_node = &mut tree;

        for depth in 1 .. (max_depth + 1) {
            let (_, _, total_nodes, pv) = Searcher::traverse_tree(current_node, current_pos);
            let score = current_node.best_score.unwrap().score();
            //let pv =  Move_::get_fen(current_node.best_move.unwrap());
            let time = Searcher::get_time_elapsed_ms(start);
            
            let mut nps = 0;
            if time > 0 {
                nps = (total_nodes as u64) * 1000 / time;
            }

            let pv_string = Searcher::get_moves_string(&pv);

            println!("info depth {} score cp {} time {} nodes {} nps {} pv {}", depth, score, time, total_nodes, nps, pv_string);
        }

        return current_node.best_move.unwrap();


    }

    fn get_time_elapsed_ms(start: SystemTime) -> u64 {
        let dur = SystemTime::now().duration_since(start).expect("SystemTime::duration_since failed");
        1000 * dur.as_secs() + (dur.subsec_millis() as u64)
    }

    fn traverse_tree(node: &mut Tree, position: &Position) -> (Option<Outcome>, Option<u32>, u32, Vec<u32>) {
        //check for stop signal here
        let mut current_nodes = 0u32;
        let mut pv: Vec<u32> = Vec::new();
        match node.sub_trees {
            Some(_) => {
                //we must go deeper
                node.best_score = None;
                node.best_move = None;
                let mut best_sub_pv: Vec<u32> = Vec::new();
                if let Some(mut sub_trees) = node.sub_trees.take() {
                    for (move_, sub_tree) in sub_trees.iter_mut() {
                        let mut pos = position.clone();
                        pos.apply_move(*move_);
                        let (sub_best_score, _, sub_nodes, sub_pv) = Searcher::traverse_tree(sub_tree, &pos);
                        current_nodes += sub_nodes;

                        //println!("found best sub move {}", Move_::get_fen(sub_best_move.unwrap())); 

                        if sub_pv.len() > 2 {
                            println!("evaluated sub pv {} {} with score {}", 
                                Move_::get_fen(*move_),
                                Searcher::get_moves_string(&sub_pv), 
                                sub_best_score.unwrap().score());
                        }

                        if Searcher::is_better_outcome(&sub_best_score, &node.best_score, position.active_color) {
                            node.best_score = sub_best_score;
                            node.best_move = Some(*move_);
                            best_sub_pv = sub_pv;
                        }

                    }
                    node.sub_trees = Some(sub_trees);
                    pv.push(node.best_move.unwrap());
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
                pv.push(node.best_move.unwrap());
            }
        }
        (node.best_score, node.best_move, current_nodes, pv)
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

