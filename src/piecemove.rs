use std::collections::HashMap;

use crate::global;
use crate::square::Square;

lazy_static! {
    static ref PIECE_MOVES: HashMap<u8, Vec<Dir>> = {
        let mut m = HashMap::new();
        m.insert(global::PIECE_KING, vec![
            Dir { mov: &Square::up, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::up_left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::up_right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_right, max_steps: 1, capture: true, silent: true}]
        );
        m.insert(global::PIECE_QUEEN, vec![
            Dir { mov: &Square::up, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::right, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::up_left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::up_right, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down_left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down_right, max_steps: 7, capture: true, silent: true}]
        );
        m.insert(global::PIECE_ROOK, vec![
            Dir { mov: &Square::up, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::right, max_steps: 7, capture: true, silent: true}]
        );
        m.insert(global::PIECE_BISHOP, vec![
            Dir { mov: &Square::up_left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::up_right, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down_left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down_right, max_steps: 7, capture: true, silent: true}]
        );
        m.insert(global::PIECE_KNIGHT, vec![
            Dir { mov: &Square::up_up_right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::up_up_left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_down_right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_down_left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::up_right_right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::up_left_left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_right_right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_left_left, max_steps: 1, capture: true, silent: true}]
        );
        m.insert(global::PIECE_PAWN_WHITE, vec![
            Dir { mov: &Square::up, max_steps: 1, capture: false, silent: true},
            Dir { mov: &Square::up_left, max_steps: 1, capture: true, silent: false},
            Dir { mov: &Square::up_right, max_steps: 1, capture: true, silent: false}]
        );
        m.insert(global::PIECE_PAWN_BLACK, vec![
            Dir { mov: &Square::down, max_steps: 1, capture: false, silent: true},
            Dir { mov: &Square::down_left, max_steps: 1, capture: true, silent: false},
            Dir { mov: &Square::down_right, max_steps: 1, capture: true, silent: false}]
        );
        m
    };
}

pub struct Dir {
    pub mov: &'static (Fn(u8) -> Option<u8> + Sync),
    pub max_steps: u8,
    pub capture: bool,
    pub silent: bool
}

pub fn get_piece_moves<'a>(piece_type: u8) -> &'a Vec<Dir> {
    &PIECE_MOVES[&piece_type]
}