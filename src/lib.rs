#![allow(dead_code)]

extern crate ctrlc;

#[macro_use]
extern crate lazy_static;
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
mod piecemove;
mod generator;
mod searchcommand;
mod searcher;
mod outcome;
mod evaluation;
mod tree;

mod tests;