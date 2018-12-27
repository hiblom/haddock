use crate::global;
use crate::position::Position;
use crate::piece::Piece;
use crate::piecemove;

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

pub fn king_is_attacked(position: &Position, current_square: u8, color: u8) -> bool {
    let king_checked_moves = piecemove::get_king_checked_moves();
    let other_color = 1 - color;

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
                        else if Piece::get_color(other_piece) != color {
                            //enemy piece
                            let mut piece_type = Piece::get_type(other_piece);
                            if piece_type == global::PIECE_PAWN {
                                piece_type |= other_color;
                            }
                            if dirs_pieces.1.contains(&piece_type) {
                                return true;
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

    false
}


