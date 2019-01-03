use std::collections::HashMap;

use crate::global;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;


use crate::position::Position;
use crate::piecetype::PieceType;
use crate::piecemove;
use crate::outcome::Outcome;
use crate::generator;

lazy_static! {
    static ref POINT_VALUE: HashMap<PieceType, i16> = {
        let mut m = HashMap::new();
        m.insert(PieceType::new_pawn(COLOR_WHITE), 100);
        m.insert(PieceType::new_pawn(COLOR_BLACK), -100);
        m.insert(PieceType::new_knight(COLOR_WHITE), 300);
        m.insert(PieceType::new_knight(COLOR_BLACK), -300);
        m.insert(PieceType::new_bishop(COLOR_WHITE), 300);
        m.insert(PieceType::new_bishop(COLOR_BLACK), -300);
        m.insert(PieceType::new_rook(COLOR_WHITE), 500);
        m.insert(PieceType::new_rook(COLOR_BLACK), -500);
        m.insert(PieceType::new_queen(COLOR_WHITE), 900);
        m.insert(PieceType::new_queen(COLOR_BLACK), -900);
        m
    };
}

pub fn is_check(position: &Position, color: u8) -> bool {
    let other_color = 1 - color;
    match position.get_king_square(color) {
        Some(s) => return find_square_attackers(position, s, other_color).len() > 0,
        _ => false
    }
}

pub fn find_square_attackers(position: &Position, current_square: u8, color: u8) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    let king_checked_moves = piecemove::get_king_checked_moves();

    for dirs_pieces in king_checked_moves {
        for dir in &dirs_pieces.0 {
            let mut square = current_square;
            for _ in 0..dir.max_steps {
                match (dir.mov)(square) {
                    Some(s) => {
                        match position.get_piece(s) {
                            None => square = s,
                            Some(other_piece) => {
                                if other_piece.has_color(color) {
                                    //enemy piece
                                    let mut piece_val = other_piece.get_type();
                                    if other_piece.is_pawn() {
                                        piece_val |= color;
                                    }
                                    if dirs_pieces.1.contains(&PieceType::new(piece_val)) {
                                        result.push(s);
                                    }
                                    break;
                                }
                                else {
                                    //friendly piece
                                    break;
                                }
                            }
                        }
                    },
                    None => break
                }

            }
        }
    }

    result
}

pub fn evaluate(position: &Position) -> Outcome {
    let check = is_check(position, position.get_active_color());
    
    let no_legal_moves_left = generator::generate_legal_moves(position).len() == 0;

    let check_mate = check && no_legal_moves_left;
    if check_mate {
        if position.get_active_color() == global::COLOR_WHITE {
            return Outcome::WhiteIsMate(1)
        }
        else {
            return Outcome::BlackIsMate(1)
        }
    }

    let stale_mate = !check && no_legal_moves_left;
    if stale_mate {
        return Outcome::DrawByStalemate
    }

    let halfmoveclock = position.get_halfmoveclock() >= global::MAX_HALFMOVECLOCK;
    if halfmoveclock {
        return Outcome::DrawByHalfmoveclock
    }

    //TODO repetition
    //TODO not enough material

    let material_value = get_material_value(position);
    Outcome::Undecided(material_value)
}

fn get_material_value(position: &Position) -> i16 {
    let mut value: i16 = 0;
    for piece in position.get_active_pieces() {
        value += POINT_VALUE[&piece.0];
    }
    value
}