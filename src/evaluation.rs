/*

use crate::global;
use crate::position::Position;
use crate::square::Square;

pub fn is_check(position: &Position, color:u8) -> bool {
    let king: u8 = global::PIECE_KING | color;
    for square in 0u8..64 {
        let piece = position.pieces[square as usize];
        if piece == king {
            return king_is_attacked(position, square, color);
        }
    }

    false
}

pub fn king_is_attacked(position: &Position, square: u8, color: u8) -> bool {
    //cannot be attacked by other king or enpassant so we skip this

    let (current_x, current_y) = Square::get_xy(square);
    let mut piece: u8;
    let other_color:u8 = 1 - color;

    //attacked by pawn
    if color == global::COLOR_WHITE {
        if current_y < 6 {
            match Square::up_left(square) {
                Some(s) => {
                    if sq_has_piece(position, s, global::PIECE_PAWN, other_color) {
                        return true;
                    }
                }
                None => ()
            }

            match Square::up_right(square) {
                Some(s) => {
                    if sq_has_piece(position, s, global::PIECE_PAWN, other_color) {
                        return true;
                    }
                }
                None => ()
            }
        }
    }
    else {
        if current_y > 1 {
            match Square::down_left(square) {
                Some(s) => {
                    if sq_has_piece(position, s, global::PIECE_PAWN, other_color) {
                        return true;
                    }
                }
                None => ()
            }

            match Square::down_right(square) {
                Some(s) => {
                    if sq_has_piece(position, s, global::PIECE_PAWN, other_color) {
                        return true;
                    }
                }
                None => ()
            }
        }
    }

    //attacked by knight



    //attacked by queen/rook/bishop

    false
}

fn sq_has_piece(position: &Position, square: u8, piece_type: u8, color: u8) -> bool {
    let piece = position.pieces[square as usize];
    return piece == piece_type | color;
}

*/