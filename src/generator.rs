use crate::position::Position;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;
use crate::piecetype;
use crate::piecetype::PieceType;
use crate::move_::Move_;
use crate::square;
use crate::square::Square;
use crate::piecemove;
use crate::evaluation;

pub fn generate_moves(position: &Position) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for (piece_type, square) in position.get_active_pieces() {
        let mut piece_moves = generate_piece_moves(position, square, piece_type);
        result.append(&mut piece_moves);
    }

    result
}

pub fn is_legal_move(position: &Position, move_: u32) -> bool {
    let color = position.get_active_color();
    //find all moves
    let moves = generate_moves(position);
    if moves.contains(&move_) {
        let mut pos = position.clone();
        pos.apply_move(move_);
        return !evaluation::is_check(&pos, color);
    }
    false
}


/*
pub fn generate_legal_moves(position: &Position) -> Vec<u32> {
    let color = position.get_active_color();
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
*/

pub fn generate_piece_moves(position: &Position, square: Square, piece_type: PieceType) -> Vec<u32> {

    let pt = piece_type.get_type();
    let piece_color = piece_type.get_color();
    match pt {
        piecetype::PIECE_KING => generate_king_moves(position, square, piece_color),
        piecetype::PIECE_QUEEN => generate_normal_piece_moves(position, square, piece_color),
        piecetype::PIECE_ROOK => generate_normal_piece_moves(position, square, piece_color),
        piecetype::PIECE_BISHOP => generate_normal_piece_moves(position, square, piece_color),
        piecetype::PIECE_KNIGHT => generate_normal_piece_moves(position, square, piece_color),
        piecetype::PIECE_PAWN => generate_pawn_moves(position, square, piece_color),
        _ => Vec::new()
    }
}

pub fn generate_normal_piece_moves(position: &Position, current_square: Square, color: u8) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    let piece;
    match position.get_piece(current_square) {
        Some(p) => piece = p,
        None => return result
    }

    let move_type = piece.get_move_type();

    let dir_target_squares = piecemove::get_prerendered_target_squares(move_type, current_square);

    for dir_sq in dir_target_squares {
        for sq in &dir_sq.squares {

            match position.get_piece(*sq) {
                None => {
                    if dir_sq.silent {
                        result.push(Move_::from_squares(current_square, *sq));
                    }
                },
                Some(other_piece) => {
                    if other_piece.has_color(color) {
                        //block by piece of same color
                        break;
                    }
                    else {
                        if dir_sq.capture {
                            result.push(Move_::from_squares(current_square, *sq));
                        }
                        break
                    }
                }
            }
        }
    }

    result
}

pub fn generate_king_moves(position: &Position, current_square: Square, color: u8) -> Vec<u32> {
    let mut result = generate_normal_piece_moves(position, current_square, color);

    //TODO king squares cannot be attacked by other color
    //castling
    if color == COLOR_WHITE {
        if position.get_castling_status(0) {
            //white K-side
            if  position.get_piece(square::F1).is_none() && 
                position.get_piece(square::G1).is_none() {
                    let mut mv:u32 = Move_::from_squares(square::E1, square::G1);
                    mv.set_castling(true);
                    result.push(mv);
            }
        }
        if position.get_castling_status(1) {
            //white Q-side
            if  position.get_piece(square::B1).is_none() && 
                position.get_piece(square::C1).is_none() && 
                position.get_piece(square::D1).is_none() {
                    let mut mv:u32 = Move_::from_squares(square::E1, square::C1);
                    mv.set_castling(true);
                    result.push(mv);
            }
        }
    }
    else {
        if position.get_castling_status(2) {
            //black K-side
            if  position.get_piece(square::F8).is_none() && 
                position.get_piece(square::G8).is_none() {
                    let mut mv:u32 = Move_::from_squares(square::E8, square::G8);
                    mv.set_castling(true);
                    result.push(mv);
            }
        }
        if position.get_castling_status(3) {
            //black Q-side
            if  position.get_piece(square::B8).is_none() && 
                position.get_piece(square::C8).is_none() && 
                position.get_piece(square::D8).is_none() {
                    let mut mv:u32 = Move_::from_squares(square::E8, square::C8);
                    mv.set_castling(true);
                    result.push(mv);
            }
        }
    }

    result
}

pub fn generate_pawn_moves(position: &Position, current_square: Square, color: u8) -> Vec<u32> {
    let mut moves = generate_normal_piece_moves(position, current_square, color);

    let (current_x, current_y) = current_square.to_xy();

    //double move
    if color == COLOR_WHITE && current_y == 1 || color == COLOR_BLACK && current_y == 6 {
        match pawn_advance(current_square, color) {
            Some(sq) => {
                if position.get_piece(sq).is_none() {
                    match pawn_advance(sq, color) {
                        Some(sq) => {
                            if position.get_piece(sq).is_none() {
                                moves.push(Move_::from_squares(current_square, sq));
                            }
                        },
                        None => ()
                    }
                }
            },
            None => ()
        }
    }

    //en-passant
    match position.get_enpassant_square() {
        None => (),
        Some(ep_sq) => {
            let (ep_x, ep_y) = ep_sq.to_xy();
            let y_to = match color {
                COLOR_WHITE => current_y + 1,
                _ => current_y - 1
            };
            if y_to == ep_y {
                if current_x > 0 {
                    let x_to = current_x - 1;
                    if x_to == ep_x {
                        let mut mv:u32 = Move_::from_squares(current_square, ep_sq);
                        mv.set_enpassant(true);
                        moves.push(mv);
                    }
                }
                if current_x < 7 {
                    let x_to = current_x + 1;
                    if x_to == ep_x {
                        let mut mv:u32 = Move_::from_squares(current_square, ep_sq);
                        mv.set_enpassant(true);
                        moves.push(mv);
                    }
                }
            }
        }
    };

    //promotion
    let mut result: Vec<u32> = Vec::new();

    for m in moves {
        let (_, sq_to) = Move_::get_squares(m);
        let (_, y_to) = sq_to.to_xy();
        if (color == COLOR_WHITE && y_to == 7) || (color == COLOR_BLACK && y_to == 0) {
            result.push(Move_::create_promo_copy(m, PieceType::new_queen(color)));
            result.push(Move_::create_promo_copy(m, PieceType::new_rook(color)));
            result.push(Move_::create_promo_copy(m, PieceType::new_bishop(color)));
            result.push(Move_::create_promo_copy(m, PieceType::new_knight(color)));
        }
        else {
            result.push(m);
        }
    }
    result
}

fn pawn_advance(square: Square, color: u8) -> Option<Square> {
    match color {
        COLOR_WHITE => square.up(),
        _ => square.down()
    }

}