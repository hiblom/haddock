#![allow(dead_code)]

extern crate ctrlc;

#[macro_use]
extern crate lazy_static;
#[macro_use]
pub mod macros;

pub mod global;
pub mod searchtype;
pub mod command;
pub mod uci;
pub mod square;
pub mod move_;
pub mod piecetype;
pub mod game;
pub mod position;
pub mod moveresult;
pub mod parser;
pub mod generator;
pub mod searchcommand;
pub mod searcher;
pub mod outcome;
pub mod evaluation;
pub mod bitboard;
pub mod moveboard;

mod tests;