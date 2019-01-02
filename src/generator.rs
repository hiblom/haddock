use crate::position::Position;
use crate::global;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;
use crate::piece::Piece;  
use crate::move_::Move_;
use crate::move_::MoveFactory;
use crate::square::Square;
use crate::piecemove;
use crate::evaluation;

pub fn generate_moves(position: &Position) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for square in 0u8..64 {
        let piece = position.pieces[square as usize];
        if piece == 0 || !Piece::has_color(piece, position.active_color) {
            continue;
        }
        let mut piece_moves = generate_piece_moves(position, square, piece);
        result.append(&mut piece_moves);
    }

    result
}

pub fn generate_legal_moves(position: &Position) -> Vec<u32> {
    let color = position.active_color;
    //find all moves
    let moves = generate_moves(position);
    
    //filter out illegal moves
    let mut legal_moves:Vec<u32> = Vec::new();
    for mv in moves {
        let mut pos = position.clone();
        pos.apply_move(mv);
        if !evaluation::is_check(&pos, color) {
            legal_moves.push(mv);
        }
    }
    legal_moves
}

pub fn generate_piece_moves(position: &Position, square: u8, piece: u8) -> Vec<u32> {

    let piece_type = Piece::get_type(piece);
    let piece_color = Piece::get_color(piece);
    match piece_type {
        global::PIECE_KING => generate_king_moves(position, square, piece_color),
        global::PIECE_QUEEN => generate_normal_piece_moves(position, square, piece_color),
        global::PIECE_ROOK => generate_normal_piece_moves(position, square, piece_color),
        global::PIECE_BISHOP => generate_normal_piece_moves(position, square, piece_color),
        global::PIECE_KNIGHT => generate_normal_piece_moves(position, square, piece_color),
        global::PIECE_PAWN => generate_pawn_moves(position, square, piece_color),
        _ => Vec::new()
    }
}

pub fn generate_normal_piece_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    //yes this is slow
    //TODO make faster, possibly with bitboards??
    let piece = position.pieces[current_square as usize];
    if piece == 0 {
        return result;
    }

    let mut piece_type = Piece::get_type(piece); //type withour color info
    
    //for pawns we need to make a distinction between black and white
    if piece_type ==  global::PIECE_PAWN {
        piece_type = piece;
    }

    let dir_target_squares = piecemove::get_prerendered_target_squares(piece_type, current_square);

    for dir_sq in dir_target_squares {
        for sq in &dir_sq.squares {
            let other_piece = position.pieces[*sq as usize];
            if other_piece == 0 {
                if dir_sq.silent {
                    result.push(MoveFactory::create(current_square, *sq));
                }
            }
            else if Piece::get_color(other_piece) != color {
                if dir_sq.capture {
                    result.push(MoveFactory::create(current_square, *sq));
                }
                break
            }
            else {
                //block by piece of same color
                break;
            }
        }
    }

    result
}

pub fn generate_king_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
    let mut result = generate_normal_piece_moves(position, current_square, color);

    //castling
    if color == COLOR_WHITE {
        if position.castling_status[0] {
            //white K-side
            if  position.pieces[global::F1 as usize] == 0 && 
                position.pieces[global::G1 as usize] == 0 {
                    result.push(MoveFactory::create(global::E1, global::G1));
            }
        }
        if position.castling_status[1] {
            //white Q-side
            if  position.pieces[global::B1 as usize] == 0 && 
                position.pieces[global::C1 as usize] == 0 && 
                position.pieces[global::D1 as usize] == 0 {
                    result.push(MoveFactory::create(global::E1, global::C1));
            }
        }
    }
    else {
        if position.castling_status[2] {
            //black K-side
            if  position.pieces[global::F8 as usize] == 0 && 
                position.pieces[global::G8 as usize] == 0 {
                    result.push(MoveFactory::create(global::E8, global::G8));
            }
        }
        if position.castling_status[3] {
            //black Q-side
            if  position.pieces[global::B8 as usize] == 0 && 
                position.pieces[global::C8 as usize] == 0 && 
                position.pieces[global::D8 as usize] == 0 {
                    result.push(MoveFactory::create(global::E8, global::C8));
            }
        }
    }

    result
}

pub fn generate_pawn_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
    let mut moves = generate_normal_piece_moves(position, current_square, color);

    let (current_x, current_y) = Square::get_xy(current_square);

    //double move
    if color == COLOR_WHITE && current_y == 1 || color == COLOR_BLACK && current_y == 6 {
        let mut sq = pawn_advance(current_square, color);
        if position.pieces[sq as usize] == 0 {
            sq = pawn_advance(sq, color);
            if position.pieces[sq as usize] == 0 {
                moves.push(MoveFactory::create(current_square, sq));
            }
        }
    }

    //en-passant
    match position.enpassant_square {
        None => (),
        Some(ep_sq) => {
            let (ep_x, ep_y) = Square::get_xy(ep_sq);
            let y_to = match color {
                COLOR_WHITE => current_y + 1,
                _ => current_y - 1
            };
            if y_to == ep_y {
                if current_x > 0 {
                    let x_to = current_x - 1;
                    if x_to == ep_x {
                        moves.push(MoveFactory::create(current_square, ep_sq));
                    }
                }
                if current_x < 7 {
                    let x_to = current_x + 1;
                    if x_to == ep_x {
                        moves.push(MoveFactory::create(current_square, ep_sq));
                    }
                }
            }
        }
    };

    //promotion
    let mut result: Vec<u32> = Vec::new();

    for m in moves {
        let (_, sq_to) = Move_::get_squares(m);
        let (_, y_to) = Square::get_xy(sq_to);
        if (color == COLOR_WHITE && y_to == 7) || (color == COLOR_BLACK && y_to == 0) {
            result.push(Move_::create_promo_copy(m, global::PIECE_QUEEN));
            result.push(Move_::create_promo_copy(m, global::PIECE_ROOK));
            result.push(Move_::create_promo_copy(m, global::PIECE_BISHOP));
            result.push(Move_::create_promo_copy(m, global::PIECE_KNIGHT));
        }
        else {
            result.push(m);
        }
    }
    result
}

fn pawn_advance(square: u8, color: u8) -> u8 {
    match color {
        COLOR_WHITE => square + 8,
        _ => square - 8
    }

}