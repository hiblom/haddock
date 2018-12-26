use crate::position::Position;
use crate::global;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;
use crate::piece::Piece;  
use crate::move_::Move_;
use crate::move_::MoveFactory;
use crate::square::Square;
use crate::square::SquareFactory;

pub fn generate_moves(position: &Position) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    //normal moves
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

pub fn generate_piece_moves(position: &Position, square: u8, piece: u8) -> Vec<u32> {

    let piece_type = Piece::get_type(piece);
    let piece_color = Piece::get_color(piece);
    match piece_type {
        global::PIECE_KING => generate_king_moves(position, square, piece_color),
        global::PIECE_QUEEN => generate_queen_moves(position, square, piece_color),
        global::PIECE_ROOK => generate_rook_moves(position, square, piece_color),
        global::PIECE_BISHOP => generate_bishop_moves(position, square, piece_color),
        global::PIECE_KNIGHT => generate_knight_moves(position, square, piece_color),
        global::PIECE_PAWN => generate_pawn_moves(position, square, piece_color),
        _ => Vec::new()
    }
}

pub fn generate_king_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    //for now: slow algorithm
    //todo find faster algorithm
    let current_x: u8 = current_square % 8;
    let current_y: u8 = current_square / 8;
    let from_square_base = (current_square as u32) << 8;

    let lower_y = match current_y {
        0 => 0,
        _ => current_y - 1
    };

    let upper_y = match current_y {
        7 => 8,
        _ => current_y + 2
    };

    let lower_x = match current_x {
        0 => 0,
        _ => current_x - 1
    };

    let upper_x = match current_x {
        7 => 8,
        _ => current_x + 2
    };

    for y in lower_y..upper_y {
        for x in lower_x..upper_x {
            if x == current_x && y == current_y {
                continue;
            }

            let to_square = y * 8 + x;
            let piece = position.pieces[to_square as usize];
            if piece == 0 || !Piece::has_color(piece, color) {
                result.push(from_square_base | to_square as u32);
            }
        }
    }

    //TODO castling
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
            println!("eval black K castling");
            //black K-side
            if  position.pieces[global::F8 as usize] == 0 && 
                position.pieces[global::G8 as usize] == 0 {
                    result.push(MoveFactory::create(global::E8, global::F8));
            }
        }
        if position.castling_status[3] {
            //black Q-side
            println!("eval black Q castling");
            if  position.pieces[global::B8 as usize] == 0 && 
                position.pieces[global::C8 as usize] == 0 && 
                position.pieces[global::D8 as usize] == 0 {
                    println!("black Q castling found");
                    result.push(MoveFactory::create(global::E8, global::C8));
            }
        }
    }

    result
}

pub fn generate_queen_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
    //for now we just make a combination of rook and bishop
    //todo find faster algorithm
    let mut result: Vec<u32> = generate_rook_moves(position, current_square, color);
    result.append(&mut generate_bishop_moves(position, current_square, color));

    result
}

pub fn generate_rook_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    //for now: slow algorithm
    //todo find faster algorithm
    let current_x: u8 = current_square % 8;
    let current_y: u8 = current_square / 8;

    //right
    for x in current_x + 1.. 8 {
        if !try_add_move(position, &mut result, current_square, x, current_y, color) {
            break;
        }
    }

    //left
    for x in (0 .. current_x).rev() {
        if !try_add_move(position, &mut result, current_square, x, current_y, color) {
            break;
        }
    }

    //up
    for y in current_y + 1.. 8 {
        if !try_add_move(position, &mut result, current_square, current_x, y, color) {
            break;
        }
    }

    //down
    for y in (0 .. current_y).rev() {
        if !try_add_move(position, &mut result, current_square, current_x, y, color) {
            break;
        }
    }

    result
}

pub fn generate_bishop_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    //for now: slow algorithm
    //todo find faster algorithm
    let current_x: u8 = current_square % 8;
    let current_y: u8 = current_square / 8;

    //up right
    let mut x = current_x;
    let mut y = current_y;
    while x < 7 && y < 7 {
        x += 1;
        y += 1;
        if !try_add_move(position, &mut result, current_square, x, y, color) {
            break;
        }
    }

    //up left
    let mut x = current_x;
    let mut y = current_y;
    while x > 0 && y < 7 {
        x -= 1;
        y += 1;
        if !try_add_move(position, &mut result, current_square, x, y, color) {
            break;
        }
    }

    //down right
    let mut x = current_x;
    let mut y = current_y;
    while x < 7 && y > 0 {
        x += 1;
        y -= 1;
        if !try_add_move(position, &mut result, current_square, x, y, color) {
            break;
        }
    }

    //down left
    let mut x = current_x;
    let mut y = current_y;
    while x > 0 && y > 0 {
        x -= 1;
        y -= 1;
        if !try_add_move(position, &mut result, current_square, x, y, color) {
            break;
        }
    }

    result
}

pub fn generate_knight_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    //for now: slow algorithm
    //todo find faster algorithm
    let current_x: u8 = current_square % 8;
    let current_y: u8 = current_square / 8;

    if current_x > 0 {
        if current_y > 1 {
            try_add_move(position, &mut result, current_square, current_x - 1, current_y - 2, color);
        }
        if current_y < 6 {
            try_add_move(position, &mut result, current_square, current_x - 1, current_y + 2, color);
        }
        if current_x > 1 {
            if current_y > 0 {
                try_add_move(position, &mut result, current_square, current_x - 2, current_y - 1, color);
            }
            if current_y < 7 {
                try_add_move(position, &mut result, current_square, current_x - 2, current_y + 1, color);
            }
        }
    }

    if current_x < 7 {
        if current_y > 1 {
            try_add_move(position, &mut result, current_square, current_x + 1, current_y - 2, color);
        }
        if current_y < 6 {
            try_add_move(position, &mut result, current_square, current_x + 1, current_y + 2, color);
        }
        if current_x < 6 {
            if current_y > 0 {
                try_add_move(position, &mut result, current_square, current_x + 2, current_y - 1, color);
            }
            if current_y < 7 {
                try_add_move(position, &mut result, current_square, current_x + 2, current_y + 1, color);
            }
        }
    }

    result
}

pub fn generate_pawn_moves(position: &Position, current_square: u8, color: u8) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    //for now: slow algorithm
    //todo find faster algorithm
    let (current_x, current_y) = Square::get_xy(current_square);

    let mut to_square: u8;
    let mut piece: u8;

    //normal move (no capture)
    let mut y = current_y;
    if y > 0 && y < 7 {
        if color == COLOR_WHITE {
            y += 1
        }
        else {
            y -= 1
        }
        to_square = SquareFactory::create(current_x, y);
        piece = position.pieces[to_square as usize];
        if piece == 0 {
            result.push(MoveFactory::create(current_square, to_square));

            if color == COLOR_WHITE && current_y == 1 || color == COLOR_BLACK && current_y == 6 {
                if color == COLOR_WHITE {
                    y += 1
                }
                else {
                    y -= 1
                }
                to_square = SquareFactory::create(current_x, y);
                piece = position.pieces[to_square as usize];
                if piece == 0 {
                    result.push(MoveFactory::create(current_square, to_square));
                }

            }
        }
    }

    //captures left/right
    y = current_y;
    let mut x;
    if y > 0 && y < 7 {
        if color == COLOR_WHITE {
            y += 1
        }
        else {
            y -= 1
        }

        if current_x > 0 {
            x = current_x - 1;
            to_square = y * 8 + x;
            piece = position.pieces[to_square as usize];
            
            let ep = (piece == 0) &&
                (color == COLOR_WHITE && current_y == 4 || color == COLOR_BLACK && current_y == 3) &&
                match position.enpassant_square {
                    Some(s) => s == to_square,
                    None => false };

            if piece != 0 && !Piece::has_color(piece, color) || ep {
                result.push(MoveFactory::create(current_square, to_square));
            }
        }

        if current_x < 7 {
            x = current_x + 1;
            to_square = y * 8 + x;
            piece = position.pieces[to_square as usize];

            let ep = (piece == 0) &&
                (color == COLOR_WHITE && current_y == 4 || color == COLOR_BLACK && current_y == 3) &&
                match position.enpassant_square {
                    Some(s) => s == to_square,
                    None => false };

            if piece != 0 && !Piece::has_color(piece, color) || ep {
                result.push(MoveFactory::create(current_square, to_square));
            }
        }
    }

    //promotions
    let end_result = find_promos(result, color);
   
    end_result
}

fn find_promos(moves: Vec<u32>, color: u8) -> Vec<u32> {
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

fn try_add_move(position: &Position, move_list: &mut Vec<u32>, current_square: u8, x:u8, y:u8, color: u8) -> bool {
    let to_square = y * 8 + x;
    let piece = position.pieces[to_square as usize];
    if piece == 0 || !Piece::has_color(piece, color) {
        let from_square_base = (current_square as u32) << 8;
        move_list.push(from_square_base | to_square as u32);
    }

    piece == 0
}

