#![allow(dead_code)]

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

mod tests;