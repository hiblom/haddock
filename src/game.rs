use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::searchtype::SearchType;
use crate::command::InputCommand;
use crate::parser;
use crate::position::Position;
use crate::searchcommand::SearchCommand;
use crate::searcher::Searcher;
use crate::move_::Move_;
use crate::generator;

pub struct Game {
    receiver: Receiver<InputCommand>,
    position: Option<Position>,
    searcher_handle: Option<thread::JoinHandle<()>>,
    searcher_channel: Option<Sender<SearchCommand>>,
    stop_signal: Arc<AtomicBool>
}

impl<'a> Game {
    pub fn new(receiver: Receiver<InputCommand>) -> Game {
        println!("Initializing game");
        Game {
            receiver: receiver,
            position: None,
            searcher_handle: None,
            searcher_channel: None,
            stop_signal: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn start(&mut self) {
        //blocking loop to handle commands
        //long taking commands should be handled in a child thread (like searching, I/O, etc)
        loop {
            let command = self.receiver.recv().unwrap();
            if !self.handle_command(&command) {
                break;
            }
        }
    }

    fn handle_command(&mut self, command: &InputCommand) -> bool {
        match command {
            InputCommand::Quit => self.handle_command_quit(),
            InputCommand::Stop => self.handle_command_stop(),
            InputCommand::Position(args) => self.handle_command_position(&args),
            InputCommand::Go(args) => self.handle_command_go(&args),
            _ => {
                println!("handle_command received other command");
                true
            }
        }
    }

    fn handle_command_quit(&mut self) -> bool {
        self.stop_signal.store(true, Ordering::Relaxed);
        self.cleanup_searcher();
        false
    }

    fn handle_command_stop(&mut self) -> bool {
        self.stop_signal.store(true, Ordering::Relaxed);
        true
    }

    fn handle_command_position(&mut self, args: &str) -> bool {
        self.position = None;

        let args_parts = args.split(" ").collect::<Vec<&str>>();

        let mut i: usize = 0;
        while i < args_parts.len() {
            match args_parts[i] {
                "startpos" => match parser::parse_startpos() {
                    Some(position) => {
                        self.position = Some(position);
                    }
                    None => {
                        println!("Error in startpos FEN");
                        return true;
                    }
                },
                "fen" => {
                    let max_fen_index;
                    match args_parts.iter().position(|&r| r == "moves") {
                        None => max_fen_index = args_parts.len() + 1,
                        Some(i) => max_fen_index = i
                    }

                    if max_fen_index < 2 {
                        println!("FEN too short");
                        return true;
                    }

                    match parser::parse_fen(&args_parts[1..max_fen_index]) {
                        Some(position) => {
                            self.position = Some(position);
                            i += max_fen_index;
                        }
                        None => {
                            println!("Error in FEN");
                            return true;
                        }
                    }
                }
                "moves" => (),
                _ => match &mut self.position {
                        Some(pos) => {
                            match Move_::from_str(args_parts[i]) {
                                Some(mv) => {
                                    let moves = generator::generate_legal_moves(pos);
                                    if moves.contains(&mv) {
                                        pos.apply_move(mv);
                                    }
                                    else {
                                        println!("{} is an illegal move!", &args_parts[i]);
                                        return true;  
                                    }
                                }, 
                                None => {
                                    println!("Error in move");
                                    return true;
                                }
                            }
                        },
                        None => return true
                    }
                }
            i += 1;
        }

        //println!("{}", self.position.unwrap());

        true
    }

    fn handle_command_go(&mut self, args: &str) -> bool {
        let args_parts = args.split(" ").collect::<Vec<&str>>();

        let mut search_type = SearchType::Infinite;

        let mut i: usize = 0;
        while i < args_parts.len() {
            match args_parts[i] {
                "infinite" => {
                    search_type = SearchType::Infinite
                },
                "depth" => {
                    i += 1;
                    let (succeeded, value) = Game::get_numeric_value(&args_parts, i);
                    if !succeeded {
                        return true;
                    }
                    search_type = SearchType::Depth(value);
                },
                "nodes" => {
                    i += 1;
                    let (succeeded, value) = Game::get_numeric_value(&args_parts, i);
                    if !succeeded {
                        return true;
                    }
                    search_type = SearchType::Nodes(value);
                },
                "movetime" => {
                    i += 1;
                    let (succeeded, value) = Game::get_numeric_value(&args_parts, i);
                    if !succeeded {
                        return true;
                    }
                    search_type = SearchType::MoveTime(value);
                },
                "wtime" => {
                    i += 1;
                    let (succeeded, value) = Game::get_numeric_value(&args_parts, i);
                    if !succeeded {
                        return true;
                    }
                    if let SearchType::CTime(ref mut wtime, _, _, _) = search_type {
                        *wtime = value;
                    } else {
                        search_type = SearchType::CTime(value, 0, 0, 0);
                    }
                },
                "btime" => {
                    i += 1;
                    let (succeeded, value) = Game::get_numeric_value(&args_parts, i);
                    if !succeeded {
                        return true;
                    }
                    if let SearchType::CTime(_, ref mut btime, _, _) = search_type {
                        *btime = value;
                    } else {
                        search_type = SearchType::CTime(0, value, 0, 0);
                    }
                },
                "winc" => {
                    i += 1;
                    let (succeeded, value) = Game::get_numeric_value(&args_parts, i);
                    if !succeeded {
                        return true;
                    }
                    if let SearchType::CTime(_, _, ref mut winc, _) = search_type {
                        *winc = value;
                    } else {
                        search_type = SearchType::CTime(0, 0, value, 0);
                    }
                },
                "binc" => {
                    i += 1;
                    let (succeeded, value) = Game::get_numeric_value(&args_parts, i);
                    if !succeeded {
                        return true;
                    }
                    if let SearchType::CTime(_, _, _, ref mut binc) = search_type {
                        *binc = value;
                    } else {
                        search_type = SearchType::CTime(0, 0, 0, value);
                    }
                },
                _ => ()
            }
            i += 1;
        }


        let sender: &Sender<SearchCommand>;
        self.setup_search();
        
        match &self.searcher_channel {
            Some(s) => sender = s,
            None => return true
        }

        sender.send(SearchCommand::FindBestMove(search_type)).
            expect("Error while sending search command");

        true
    }

    fn get_numeric_value(args_parts: &Vec<&str>, i: usize) -> (bool, u64) {
        if args_parts.len() < i {
            return (false, 0);
        }
        match args_parts[i].parse::<u64>() {
            Ok(n) => {
                (true, n)
            },
            Err(_) => {
                println!("Could not parse number");
                (false, 0)
            }
        }
    }

    fn setup_search(&mut self) {
        //cleanup current search, if needed
        self.stop_signal.store(false, Ordering::Relaxed);
        self.cleanup_searcher();

        let (sender_request, receiver_request): (Sender<SearchCommand>, Receiver<SearchCommand>) = mpsc::channel();
    
        let position_clone: Position;
        
        match self.position {
            Some(p) => position_clone = p.clone(),
            None => {
                println!("Cannot setup search without position");
                return
            }
        }

        let stop_signal_clone = self.stop_signal.clone();

        self.searcher_handle = Some(thread::spawn(move || {
            let mut searcher = Searcher::new(receiver_request, position_clone, stop_signal_clone);
            searcher.start();
        }));

        self.searcher_channel = Some(sender_request);
    }

    fn cleanup_searcher(&mut self) {
        match &self.searcher_channel {
            Some(sc) => {
                sc.send(SearchCommand::Quit).expect("Error while sending search command");
                if let Some(sh) = self.searcher_handle.take() {
                    sh.join().expect("Error while synchronizing with search thread");
                    self.searcher_handle = None;
                    self.searcher_channel = None
                }
            }
            None => ()
        }
    }
}
