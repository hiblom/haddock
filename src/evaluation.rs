use std::collections::HashMap;

use crate::global;
use crate::position::Position;
use crate::piece::Piece;
use crate::piecemove;
use crate::outcome::Outcome;
use crate::generator;

lazy_static! {
    static ref POINT_VALUE: HashMap<u8, i16> = {
        let mut m = HashMap::new();
        m.insert(global::PIECE_PAWN, 100);
        m.insert(global::PIECE_KNIGHT, 300);
        m.insert(global::PIECE_BISHOP, 300);
        m.insert(global::PIECE_ROOK, 500);
        m.insert(global::PIECE_QUEEN, 900);
        m.insert(global::PIECE_KING, 10000);
        m
    };
}

pub fn is_check(position: &Position, color: u8) -> bool {
    let other_color = 1 - color;
    let king: u8 = global::PIECE_KING | color;
    for square in 0u8..64 {
        let piece = position.pieces[square as usize];
        if piece == king {
            return find_square_attackers(position, square, other_color).len() > 0;
        }
    }

    false
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
                        let other_piece = position.pieces[s as usize];
                        if other_piece == 0 {
                            square = s;
                        }
                        else if Piece::get_color(other_piece) == color {
                            //enemy piece
                            let mut piece_type = Piece::get_type(other_piece);
                            if piece_type == global::PIECE_PAWN {
                                piece_type |= color;
                            }
                            if dirs_pieces.1.contains(&piece_type) {
                                result.push(s);
                            }
                            break;
                        }
                        else {
                            //friendly piece
                            break;
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
    let check = is_check(position, position.active_color);
    
    let no_legal_moves_left = generator::generate_legal_moves(position).len() == 0;

    let check_mate = check && no_legal_moves_left;
    if check_mate {
        if position.active_color == global::COLOR_WHITE {
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

    let halfmoveclock = position.halfmoveclock >= global::MAX_HALFMOVECLOCK;
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
    for square in 0u8..64 {
        let piece = position.pieces[square as usize];
        if piece != 0 {
            let piece_value = POINT_VALUE[&Piece::get_type(piece)];
            if Piece::get_color(piece) == global::COLOR_WHITE {
                value += piece_value
            }
            else {
                value -= piece_value;
            }
        }
    }
    value
}