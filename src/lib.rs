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
pub mod piece;
pub mod game;
pub mod position;
pub mod parser;
pub mod piecemove;
pub mod generator;
pub mod searchcommand;
pub mod searcher;
pub mod outcome;
pub mod evaluation;
pub mod tree;

mod tests;