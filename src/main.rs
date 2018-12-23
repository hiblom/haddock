extern crate ctrlc;

#[macro_use]

mod macros;

mod global;
mod command;
mod uci;
mod square;
mod move_;
mod piece;
mod game;
mod position;
mod parser;
mod generator;
mod searchcommand;
mod searcher;

use std::io;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use crate::command::InputCommand;
use crate::game::Game;

fn main() {
    println!("Welcome to Haddock Chess Engine");

    //initialize game...
    //start thread to initialize
    //communication with game will be done async
    //prepare move matrices???? -> omg!!

    let (game_handle, game_channel) = setup_game();
    let (_console_handle, input_channel) = setup_input();

    println!("Please input UCI command");

    //main loop to receive commands
    loop {
        let line: String;
        match input_channel.recv() {
            Ok(s) => line = s,
            Err(e) => {
                println!("Error while receiving input : {}", e);
                break;
            }
        }

        let command = uci::parse(&line);
        
        let stay = match command {
            Some(InputCommand::Quit) => {
                false
            },
            Some(InputCommand::Position(args)) => {
                match game_channel.send(InputCommand::Position(args)) {
                    Ok(_) => (),
                    Err(e) => println!("Error sending command : {}", e)
                }
                true
            },
            Some(InputCommand::Go) => {
                match game_channel.send(InputCommand::Go) {
                    Ok(_) => (),
                    Err(e) => println!("Error sending command : {}", e)
                }
                true
            },
            Some(c) => {
                let result = command::send_command(c);
                if !result.message.is_empty() {
                    print!("{}", &result.message)
                }
                result.stay
            }
            None => {
                println!("info string unknown command");
                true
            }
        };

        if !stay {
            break;
        }
    }

    // wait till game thread finished
    match game_channel.send(InputCommand::Quit) {
        Ok(_) => (),
        Err(e) => println!("Error during cleanup : {}", e)
    }
    game_handle.join().unwrap();

    //terminate console thread how??
    println!("Goodbye");
}

fn setup_game() -> ( thread::JoinHandle<()>, Sender<InputCommand> ) {
    let (sender, receiver): (Sender<InputCommand>, Receiver<InputCommand>) = mpsc::channel();
    
    let game_handle = thread::spawn(move || {
        let mut game = Game::new(receiver);
        game.start();
    });

    (game_handle, sender)
}

fn setup_input () -> ( thread::JoinHandle<()>, Receiver<String> ) {
    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

    //input via console
    //this process has to clean itself up, read_line is a blocking command
    let sender_clone = sender.clone();
    let console_handle = thread::spawn(move || {
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("Error reading line from console");
            sender_clone.send(line).expect("Error sending string over channel");
        }
    });

    //input ctrl c
    //NOTE under cargo, this doesn't work properly, but it's working fine when run directly
    let sender_clone = sender.clone();
    ctrlc::set_handler(move || {
        println!("ctrl-c received");
        sender_clone.send("quit".to_string()).expect("Error sending string over channel");
    }).expect("Error setting ctrl-c handler");

    (console_handle, receiver)
}