use crate::global;
use crate::position::Position;
use crate::piece::Piece;

pub fn parse_startpos() -> Option<Position> {
    let fen_parts = global::FEN_STARTPOS.split(" ").collect::<Vec<&str>>();
    parse_fen(&fen_parts)
}

pub fn parse_fen(fen_parts: &[&str]) -> Option<Position> {
    if fen_parts.len() != 6 {
        return None;
    }

    let mut position = Position::new();

    if !parse_fen_pieces(&mut position, fen_parts[0]) {
        return None;
    }

    if !parse_fen_color(&mut position, fen_parts[1]) {
        return None;
    }

    if !parse_fen_castling(&mut position, fen_parts[2]) {
        return None;
    }

    if !parse_fen_enpassant(&mut position, fen_parts[3]) {
        return None;
    }

    if !parse_fen_halfmoveclock(&mut position, fen_parts[4]) {
        return None;
    }

    if !parse_fen_fullmovenumber(&mut position, fen_parts[5]) {
        return None;
    }

    Some(position)
}

fn parse_fen_pieces(position: &mut Position, fen_pieces: &str) -> bool {
    let rows = fen_pieces.split("/").collect::<Vec<&str>>();
    if rows.len() != 8 {
        return false;
    }

    for i in 0u32..8 {
        let y = 7 - i;
        let mut x: u32 = 0;
        for c in rows[i as usize].chars() {
            if !c.is_ascii() {
                continue;
            }
            if x > 7 {
                return false;
            }
            match c.to_digit(10) {
                Some(d) => {
                    x += d;
                    continue;
                },
                None => ()
            }
            match Piece::from_char(c) {
                Some(piece) => {
                    position.pieces[(y * 8 + x) as usize] = piece;
                    x+= 1;
                },
                None => return false
            }
        }
    }

    true
}

fn parse_fen_color(position: &mut Position, fen_color: &str) -> bool {
    let char_color_map = hashmap! {
    "w" => 0u8,
    "W" => 0,
    "b" => 1,
    "B" => 1};

    if fen_color.len() != 1 {
        return false;
    }

    match char_color_map.get(&fen_color) { 
        Some(c) => {
            position.active_color = *c;
            true
        },
        _ => false
    }
}

fn parse_fen_castling(position: &mut Position, fen_castling: &str) -> bool {
    if fen_castling.len() > 4 {
        return false;
    }

    position.castling_status[0] = fen_castling.contains('k');
    position.castling_status[1] = fen_castling.contains('q');
    position.castling_status[2] = fen_castling.contains('K');
    position.castling_status[3] = fen_castling.contains('Q');

    true
}

fn parse_fen_enpassant(position: &mut Position, fen_enpassant: &str) -> bool {
    if fen_enpassant == "-" {
        return true;
    }

    let square = parse_square(fen_enpassant);
    match square {
        Some(found_square) => position.enpassant_square = Some(found_square),
        None => return false
    }

    true
}

fn parse_fen_halfmoveclock(position: &mut Position, fen_halfmoveclock: &str) -> bool {
    match fen_halfmoveclock.parse::<u32>() {
        Ok(n) => position.halfmoveclock = n,
        Err(_) => return false
    }
    true
}

fn parse_fen_fullmovenumber(position: &mut Position, fen_fullmovenumber: &str) -> bool {
    match fen_fullmovenumber.parse::<u32>() {
        Ok(found_u32) => position.fullmovenumber = found_u32,
        Err(_) => return false
    }
    true
}

fn parse_square(square_string: &str) -> Option<u8> {
    let char_square_x_map = hashmap! {
    'a' => 0u8,
    'b' => 1,
    'c' => 2,
    'd' => 3,
    'e' => 4,
    'f' => 5,
    'g' => 6,
    'h' => 7,
    'A' => 0,
    'B' => 1,
    'C' => 2,
    'D' => 3,
    'E' => 4,
    'F' => 5,
    'G' => 6,
    'H' => 7};

    let char_square_y_map = hashmap! {
    '1' => 0u8,
    '2' => 1,
    '3' => 2,
    '4' => 3,
    '5' => 4,
    '6' => 5,
    '7' => 6,
    '8' => 7};

    if square_string.len() != 2 {
        return None;
    }

    let x; 
    let char_x;
    match square_string.chars().nth(0) {
        Some(c) => char_x = c,
        None => return None
    }
    match char_square_x_map.get(&char_x) {
        Some(found_x) => x = *found_x,
        None => return None
    }

    let y;
    let char_y;
    match square_string.chars().nth(1) {
        Some(c) => char_y = c,
        None => return None
    }
    match char_square_y_map.get(&char_y) {
        Some(found_y) => y = *found_y,
        None => return None
    }

    //println!("parsed square {}, {}", x, y);

    Some(y * 8 + x)
}

pub fn parse_move(move_string: &str) -> Option<u16> {
    if move_string.len() != 4 {
        return None;
    }

    let mut result:u16;

    match parse_square(&move_string[..2]) {
        Some(sq) => result = sq as u16,
        None => return None
    }

    result <<= 8;
    
    match parse_square(&move_string[2..]) {
        Some(sq) => result |= sq as u16,
        None => return None
    }

    Some(result)
}