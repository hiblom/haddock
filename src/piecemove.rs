use std::collections::HashMap;

use crate::global;
use crate::square::Square;

lazy_static! {
    static ref PRERENDERED_MOVES: HashMap<u8, Vec<Vec<DirectionSquares>>> = {
        println!("Prerendering moves");

        let mut map = HashMap::new();

        map.insert(global::PIECE_KING, generate_empty_board_moves(global::PIECE_KING));
        map.insert(global::PIECE_QUEEN, generate_empty_board_moves(global::PIECE_QUEEN));
        map.insert(global::PIECE_ROOK, generate_empty_board_moves(global::PIECE_ROOK));
        map.insert(global::PIECE_BISHOP, generate_empty_board_moves(global::PIECE_BISHOP));
        map.insert(global::PIECE_KNIGHT, generate_empty_board_moves(global::PIECE_KNIGHT));
        map.insert(global::PIECE_PAWN_WHITE, generate_empty_board_moves(global::PIECE_PAWN_WHITE));
        map.insert(global::PIECE_PAWN_BLACK, generate_empty_board_moves(global::PIECE_PAWN_BLACK));

        map
    };


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

    static ref KING_CHECKED: Vec<(Vec<Dir>, Vec<u8>)> = {
        vec![
            (
                vec![
                    Dir { mov: &Square::up_left, max_steps: 1, capture: true, silent: false},
                    Dir { mov: &Square::up_right, max_steps: 1, capture: true, silent: false}],
                vec![global::PIECE_PAWN_BLACK, global::PIECE_KING, global::PIECE_BISHOP, global::PIECE_QUEEN]
            ),
            (
                vec![
                    Dir { mov: &Square::down_left, max_steps: 1, capture: true, silent: false},
                    Dir { mov: &Square::down_right, max_steps: 1, capture: true, silent: false}],
                vec![global::PIECE_PAWN_WHITE, global::PIECE_KING, global::PIECE_BISHOP, global::PIECE_QUEEN]
            ),
            (
                vec![
                    Dir { mov: &Square::up, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::down, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::left, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::right, max_steps: 1, capture: true, silent: true}],
                vec![global::PIECE_KING, global::PIECE_ROOK, global::PIECE_QUEEN]
            ),
            (
                vec![
                    Dir { mov: &Square::up, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::down, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::left, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::right, max_steps: 7, capture: true, silent: true}],
                vec![global::PIECE_ROOK, global::PIECE_QUEEN]
            ),
            (
                vec![
                    Dir { mov: &Square::up_left, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::up_right, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::down_left, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::down_right, max_steps: 7, capture: true, silent: true}],
                vec![global::PIECE_BISHOP, global::PIECE_QUEEN]
            ),
            (
                vec![
                    Dir { mov: &Square::up_up_right, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::up_up_left, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::down_down_right, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::down_down_left, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::up_right_right, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::up_left_left, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::down_right_right, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::down_left_left, max_steps: 1, capture: true, silent: true}],
                vec![global::PIECE_KNIGHT]
            )
        ]
    };
}


pub struct Dir {
    pub mov: &'static (Fn(u8) -> Option<u8> + Sync),
    pub max_steps: u8,
    pub capture: bool,
    pub silent: bool
}

pub struct DirectionSquares {
    pub squares: Vec<u8>,
    pub capture: bool,
    pub silent: bool
}

pub fn get_piece_moves<'a>(piece_type: u8) -> &'a Vec<Dir> {
    &PIECE_MOVES[&piece_type]
}

pub fn get_king_checked_moves<'a>() -> &'a Vec<(Vec<Dir>, Vec<u8>)> {
    &KING_CHECKED
}

pub fn get_prerendered_target_squares<'a>(piece_type: u8, square: u8) -> &'a Vec<DirectionSquares> {
    &PRERENDERED_MOVES[&piece_type][square as usize]
}

fn generate_empty_board_moves(piece_type: u8) -> Vec<Vec<DirectionSquares>> {
    let mut result: Vec<Vec<DirectionSquares>> = Vec::with_capacity(64);
    for sq in 0u8..64 {
        let target_squares = generate_empty_board_squares_moves(sq, piece_type);
        result.push(target_squares);
    }
    result
}

fn generate_empty_board_squares_moves(current_square: u8, piece_type: u8) -> Vec<DirectionSquares> {
    let piece_moves = get_piece_moves(piece_type);

    let mut result: Vec<DirectionSquares> = Vec::with_capacity(piece_moves.len());

    for dir in piece_moves {
        let mut dir_sq = DirectionSquares{squares: Vec::new(), capture: dir.capture, silent: dir.silent };
        let mut square = current_square;
        for _ in 0..dir.max_steps {
            match (dir.mov)(square) {
                Some(s) => {
                    dir_sq.squares.push(s);
                    square = s;
                },
                None => break
            }
        }
        result.push(dir_sq);
    }

    result
}