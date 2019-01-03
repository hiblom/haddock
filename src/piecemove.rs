use std::collections::HashMap;

use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;

use crate::square::Square;
use crate::piecetype::PieceType;

lazy_static! {
    static ref PRERENDERED_MOVES: HashMap<PieceType, Vec<Vec<DirectionSquares>>> = {
        println!("Prerendering moves");

        let mut map = HashMap::new();

        map.insert(PieceType::new_king(COLOR_WHITE), generate_empty_board_moves(PieceType::new_king(COLOR_WHITE)));
        map.insert(PieceType::new_queen(COLOR_WHITE), generate_empty_board_moves(PieceType::new_queen(COLOR_WHITE)));
        map.insert(PieceType::new_rook(COLOR_WHITE), generate_empty_board_moves(PieceType::new_rook(COLOR_WHITE)));
        map.insert(PieceType::new_bishop(COLOR_WHITE), generate_empty_board_moves(PieceType::new_bishop(COLOR_WHITE)));
        map.insert(PieceType::new_knight(COLOR_WHITE), generate_empty_board_moves(PieceType::new_knight(COLOR_WHITE)));
        map.insert(PieceType::new_pawn(COLOR_WHITE), generate_empty_board_moves(PieceType::new_pawn(COLOR_WHITE)));
        map.insert(PieceType::new_pawn(COLOR_BLACK), generate_empty_board_moves(PieceType::new_pawn(COLOR_BLACK)));

        map
    };


    static ref PIECE_MOVES: HashMap<PieceType, Vec<Dir>> = {
        let mut m = HashMap::new();
        m.insert(PieceType::new_king(COLOR_WHITE), vec![
            Dir { mov: &Square::up, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::up_left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::up_right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_right, max_steps: 1, capture: true, silent: true}]
        );
        m.insert(PieceType::new_queen(COLOR_WHITE), vec![
            Dir { mov: &Square::up, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::right, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::up_left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::up_right, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down_left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down_right, max_steps: 7, capture: true, silent: true}]
        );
        m.insert(PieceType::new_rook(COLOR_WHITE), vec![
            Dir { mov: &Square::up, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::right, max_steps: 7, capture: true, silent: true}]
        );
        m.insert(PieceType::new_bishop(COLOR_WHITE), vec![
            Dir { mov: &Square::up_left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::up_right, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down_left, max_steps: 7, capture: true, silent: true},
            Dir { mov: &Square::down_right, max_steps: 7, capture: true, silent: true}]
        );
        m.insert(PieceType::new_knight(COLOR_WHITE), vec![
            Dir { mov: &Square::up_up_right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::up_up_left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_down_right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_down_left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::up_right_right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::up_left_left, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_right_right, max_steps: 1, capture: true, silent: true},
            Dir { mov: &Square::down_left_left, max_steps: 1, capture: true, silent: true}]
        );
        m.insert(PieceType::new_pawn(COLOR_WHITE), vec![
            Dir { mov: &Square::up, max_steps: 1, capture: false, silent: true},
            Dir { mov: &Square::up_left, max_steps: 1, capture: true, silent: false},
            Dir { mov: &Square::up_right, max_steps: 1, capture: true, silent: false}]
        );
        m.insert(PieceType::new_pawn(COLOR_BLACK), vec![
            Dir { mov: &Square::down, max_steps: 1, capture: false, silent: true},
            Dir { mov: &Square::down_left, max_steps: 1, capture: true, silent: false},
            Dir { mov: &Square::down_right, max_steps: 1, capture: true, silent: false}]
        );
        m
    };

    static ref KING_CHECKED: Vec<(Vec<Dir>, Vec<PieceType>)> = {
        vec![
            (
                vec![
                    Dir { mov: &Square::up_left, max_steps: 1, capture: true, silent: false},
                    Dir { mov: &Square::up_right, max_steps: 1, capture: true, silent: false}],
                vec![
                    PieceType::new_pawn(COLOR_BLACK), 
                    PieceType::new_king(COLOR_WHITE), 
                    PieceType::new_bishop(COLOR_WHITE), 
                    PieceType::new_queen(COLOR_WHITE)]
            ),
            (
                vec![
                    Dir { mov: &Square::down_left, max_steps: 1, capture: true, silent: false},
                    Dir { mov: &Square::down_right, max_steps: 1, capture: true, silent: false}],
                vec![
                    PieceType::new_pawn(COLOR_WHITE), 
                    PieceType::new_king(COLOR_WHITE), 
                    PieceType::new_bishop(COLOR_WHITE), 
                    PieceType::new_queen(COLOR_WHITE)]
            ),
            (
                vec![
                    Dir { mov: &Square::up, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::down, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::left, max_steps: 1, capture: true, silent: true},
                    Dir { mov: &Square::right, max_steps: 1, capture: true, silent: true}],
                vec![
                    PieceType::new_king(COLOR_WHITE), 
                    PieceType::new_rook(COLOR_WHITE), 
                    PieceType::new_queen(COLOR_WHITE)]
            ),
            (
                vec![
                    Dir { mov: &Square::up, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::down, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::left, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::right, max_steps: 7, capture: true, silent: true}],
                vec![
                    PieceType::new_rook(COLOR_WHITE), 
                    PieceType::new_queen(COLOR_WHITE)]
            ),
            (
                vec![
                    Dir { mov: &Square::up_left, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::up_right, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::down_left, max_steps: 7, capture: true, silent: true},
                    Dir { mov: &Square::down_right, max_steps: 7, capture: true, silent: true}],
                vec![
                    PieceType::new_bishop(COLOR_WHITE), 
                    PieceType::new_queen(COLOR_WHITE)]
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
                vec![
                    PieceType::new_knight(COLOR_WHITE)]
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

pub fn get_piece_moves<'a>(piece_type: PieceType) -> &'a Vec<Dir> {
    &PIECE_MOVES[&piece_type]
}

pub fn get_king_checked_moves<'a>() -> &'a Vec<(Vec<Dir>, Vec<PieceType>)> {
    &KING_CHECKED
}

pub fn get_prerendered_target_squares<'a>(piece_type: PieceType, square: u8) -> &'a Vec<DirectionSquares> {
    &PRERENDERED_MOVES[&piece_type][square as usize]
}

fn generate_empty_board_moves(piece_type: PieceType) -> Vec<Vec<DirectionSquares>> {
    let mut result: Vec<Vec<DirectionSquares>> = Vec::with_capacity(64);
    for sq in 0u8..64 {
        let target_squares = generate_empty_board_squares_moves(sq, piece_type);
        result.push(target_squares);
    }
    result
}

fn generate_empty_board_squares_moves(current_square: u8, piece_type: PieceType) -> Vec<DirectionSquares> {
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