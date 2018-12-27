extern crate rand;

use rand::Rng;
use std::sync::mpsc::Receiver;
use crate::searchcommand::SearchCommand;
use crate::position::Position;
use crate::generator;
use crate::move_::Move_;

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
        let legal_moves = generator::generate_legal_moves(&self.base_position);

        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0, legal_moves.len());

        println!("bestmove {}", Move_::get_fen(legal_moves[i]));
        //TODO OPTIMIZE
    }
}

