use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

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
    searcher_channel: Option<Sender<SearchCommand>>
}

impl Game {
    pub fn new(receiver: Receiver<InputCommand>) -> Game {
        println!("Initializing game");
        Game {
            receiver: receiver,
            position: None,
            searcher_handle: None,
            searcher_channel: None
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
            InputCommand::Position(args) => self.handle_command_position(&args),
            InputCommand::Go(args) => self.handle_command_go(&args),
            _ => {
                println!("handle_command received other command");
                true
            }
        }
    }

    fn handle_command_quit(&mut self) -> bool {
        self.cleanup_searcher();
        println!("Shutting down game");
        false
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

        let mut wtime = 0;
        let mut btime = 0;
        let mut winc = 0;
        let mut binc = 0;

        let mut i: usize = 0;
        while i < args_parts.len() {
            match args_parts[i] {
                "wtime" => {
                    if args_parts.len() > i {
                        match args_parts[i + 1].parse::<u64>() {
                            Ok(n) => {
                                wtime = n;
                                i += 1;
                            },
                            Err(_) => {
                                println!("Could not parse wtime");
                                return true;
                            }
                        }
                    }
                },
                "btime" => {
                    if args_parts.len() > i {
                        match args_parts[i + 1].parse::<u64>() {
                            Ok(n) => {
                                btime = n;
                                i += 1;
                            },
                            Err(_) => {
                                println!("Could not parse btime");
                                return true;
                            }
                        }
                    }
                },
                "winc" => {
                    if args_parts.len() > i {
                        match args_parts[i + 1].parse::<u64>() {
                            Ok(n) => {
                                winc = n;
                                i += 1;
                            },
                            Err(_) => {
                                println!("Could not parse winc");
                                return true;
                            }
                        }
                    }
                },
                "binc" => {
                    if args_parts.len() > i {
                        match args_parts[i + 1].parse::<u64>() {
                            Ok(n) => {
                                binc = n;
                                i += 1;
                            },
                            Err(_) => {
                                println!("Could not parse binc");
                                return true;
                            }
                        }
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

        sender.send(SearchCommand::FindBestMove(wtime, btime, winc, binc)).
            expect("Error while sending search command");

        true
    }

    fn setup_search(&mut self) {
        //cleanup current search, if needed
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

        self.searcher_handle = Some(thread::spawn(move || {
            let mut searcher = Searcher::new(receiver_request, position_clone);
            searcher.start();
        }));

        self.searcher_channel = Some(sender_request);
    }

    fn cleanup_searcher(&mut self) {
        match &self.searcher_channel {
            Some(sc) => {
                sc.send(SearchCommand::Quit).expect("Error while sending search command");
                if let Some(sh) = self.searcher_handle.take() { //take is fucking awesome!!!
                    sh.join().expect("Error while synchronizing with search thread");
                    self.searcher_handle = None;
                    self.searcher_channel = None
                }
            }
            None => ()
        }
    }
}
