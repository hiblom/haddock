use crate::global;
use crate::position::Position;
use crate::piece::Piece;
use crate::square::Square;
use crate::square::SquareFactory;

pub fn parse_startpos() -> Option<Position> {
    let fen_parts = global::FEN_STARTPOS.split(" ").collect::<Vec<&str>>();
    parse_fen(&fen_parts)
}

pub fn parse_fen(fen_parts: &[&str]) -> Option<Position> {
    if fen_parts.len() < 2 {
        return None;
    }

    let adj_fen_parts = apply_fen_defaults(fen_parts);

    let mut position = Position::new();

    if !parse_fen_pieces(&mut position, &adj_fen_parts[0]) {
        return None;
    }

    if !parse_fen_color(&mut position, &adj_fen_parts[1]) {
        return None;
    }

    if !parse_fen_castling(&mut position, &adj_fen_parts[2]) {
        return None;
    }

    if !parse_fen_enpassant(&mut position, &adj_fen_parts[3]) {
        return None;
    }

    if !parse_fen_halfmoveclock(&mut position, &adj_fen_parts[4]) {
        return None;
    }

    if !parse_fen_fullmovenumber(&mut position, &adj_fen_parts[5]) {
        return None;
    }

    Some(position)
}

fn apply_fen_defaults(fen_parts: &[&str]) -> [String; 6] {
    let mut result: [String; 6] = Default::default();

    result[0] = fen_parts[0].to_string();
    
    if fen_parts.len() <= 1 {
        result[1] = "w".to_string();
    }
    else {
        result[1] = fen_parts[1].to_string();
    }

    if fen_parts.len() <= 2 {
        result[2] = "-".to_string();
    }
    else {
        result[2] = fen_parts[2].to_string();
    }

    if fen_parts.len() <= 3 {
        result[3] = "-".to_string();
    }
    else {
        result[3] = fen_parts[3].to_string();
    }

    if fen_parts.len() <= 4 {
        result[4] = "0".to_string();
    }
    else {
        result[4] = fen_parts[4].to_string();
    }

    if fen_parts.len() <= 5 {
        result[5] = "1".to_string();
    }
    else {
        result[5] = fen_parts[5].to_string();
    }

    result
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
                    position.pieces[SquareFactory::create(x, y) as usize] = piece;
                    x+= 1;
                },
                None => return false
            }
        }
    }

    true
}

fn parse_fen_color(position: &mut Position, fen_color: &str) -> bool {
    if fen_color.len() != 1 {
        return false;
    }

    let lc = fen_color.to_ascii_lowercase();
    if lc == "w" {
        position.active_color = global::COLOR_WHITE;
        return true;
    }
    else if lc == "b" {
        position.active_color = global::COLOR_BLACK;
        return true;
    }

    return false;
}

fn parse_fen_castling(position: &mut Position, fen_castling: &str) -> bool {
    if fen_castling.len() > 4 {
        return false;
    }

    position.castling_status[0] = fen_castling.contains('K');
    position.castling_status[1] = fen_castling.contains('Q');
    position.castling_status[2] = fen_castling.contains('k');
    position.castling_status[3] = fen_castling.contains('q');

    true
}

fn parse_fen_enpassant(position: &mut Position, fen_enpassant: &str) -> bool {
    if fen_enpassant == "-" {
        return true;
    }

    let square = Square::from_str(fen_enpassant);

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