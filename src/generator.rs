use crate::position::Position;
use crate::global;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;
use crate::piecetype;
use crate::piecetype::PieceType;
use crate::move_::Move_;
use crate::move_::MoveFactory;
use crate::square::Square;
use crate::piecemove;
use crate::evaluation;

pub fn generate_moves(position: &Position) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for (piece_type, square) in position.get_active_pieces() {
        let mut piece_moves = generate_piece_moves(position, square, piece_type);
        result.append(&mut piece_moves);
    }

    /*
    for square in 0u8..64 {
        let piece = position.get_piece(square);
        if piece == 0 || !Piece::has_color(piece, position.get_active_color()) {
            continue;
        }
        let mut piece_moves = generate_piece_moves(position, square, piece);
        result.append(&mut piece_moves);
    }
    */

    result
}

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

pub fn generate_piece_moves(position: &Position, square: u8, piece_type: PieceType) -> Vec<u32> {

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

pub fn generate_normal_piece_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
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
                        result.push(MoveFactory::create(current_square, *sq));
                    }
                },
                Some(other_piece) => {
                    if other_piece.has_color(color) {
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
        }
    }

    result
}

pub fn generate_king_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
    let mut result = generate_normal_piece_moves(position, current_square, color);

    //castling
    if color == COLOR_WHITE {
        if position.get_castling_status(0) {
            //white K-side
            if  position.get_piece(global::F1).is_none() && 
                position.get_piece(global::G1).is_none() {
                    result.push(MoveFactory::create(global::E1, global::G1));
            }
        }
        if position.get_castling_status(1) {
            //white Q-side
            if  position.get_piece(global::B1).is_none() && 
                position.get_piece(global::C1).is_none() && 
                position.get_piece(global::D1).is_none() {
                    result.push(MoveFactory::create(global::E1, global::C1));
            }
        }
    }
    else {
        if position.get_castling_status(2) {
            //black K-side
            if  position.get_piece(global::F8).is_none() && 
                position.get_piece(global::G8).is_none() {
                    result.push(MoveFactory::create(global::E8, global::G8));
            }
        }
        if position.get_castling_status(3) {
            //black Q-side
            if  position.get_piece(global::B8).is_none() && 
                position.get_piece(global::C8).is_none() && 
                position.get_piece(global::D8).is_none() {
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
        if position.get_piece(sq).is_none() {
            sq = pawn_advance(sq, color);
            if position.get_piece(sq).is_none() {
                moves.push(MoveFactory::create(current_square, sq));
            }
        }
    }

    //en-passant
    match position.get_enpassant_square() {
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

fn pawn_advance(square: u8, color: u8) -> u8 {
    match color {
        COLOR_WHITE => square + 8,
        _ => square - 8
    }

}