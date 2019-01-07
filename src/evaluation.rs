use std::collections::HashMap;

use crate::global;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;
use crate::position::Position;
use crate::piecetype::PieceType;
use crate::outcome::Outcome;
use crate::generator::Generator;

lazy_static! {
    static ref POINT_VALUE: HashMap<PieceType, i32> = {
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
        m.insert(PieceType::new_king(COLOR_WHITE), 0);
        m.insert(PieceType::new_king(COLOR_BLACK), 0);
        m
    };
}

pub fn evaluate(position: &Position, depth: i32) -> Outcome {
    //check status of other king. when check, then the outcome is illegal
    let color = position.get_active_color();
    let other_color = 1 - color;

    if Generator::new(position).is_check(other_color) {
        return Outcome::Illegal(depth)
    }

    /*
    let check = is_check(position, color);
    
    let no_legal_moves_left = generator::generate_legal_moves(position).len() == 0;

    let check_mate = check && no_legal_moves_left;
    if check_mate {
        if color == global::COLOR_WHITE {
            return Outcome::WhiteIsMate(0)
        }
        else {
            return Outcome::BlackIsMate(0)
        }
    }

    let stale_mate = !check && no_legal_moves_left;
    if stale_mate {
        return Outcome::DrawByStalemate
    }
    */

    let halfmoveclock = position.get_halfmoveclock() >= global::MAX_HALFMOVECLOCK;
    if halfmoveclock {
        return Outcome::DrawByHalfmoveclock(depth)
    }

    //TODO repetition
    //TODO not enough material

    let material_value = get_material_value(position);
    Outcome::Undecided(depth, material_value)
}

fn get_material_value(position: &Position) -> i32 {
    let mut value: i32 = 0;
    for piece in position.get_all_active_pieces() {
        value += POINT_VALUE[&piece.0];
    }
    value
}