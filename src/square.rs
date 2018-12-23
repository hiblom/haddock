use std::marker;

const CHAR_BASE: u8 = 97;
const MASK: u8 = 0b0000_0111;

pub trait Square {
    fn new(value: Self) -> Self;
    fn from_str(value: &str) -> Option<Self> where Self: marker::Sized;
    fn from_xy(x: u8, y: u8) -> Self;
    fn get_fen(self) -> String;
}

impl Square for u8 {
    fn new(value: u8) -> u8 {
        value
    }

    fn from_str(value: &str) -> Option<u8> {
        if value.len() != 2 {
            return None;
        }
        
        let mut x: u8;
        match &value.to_lowercase().chars().nth(0) {
            Some(c) => {
                x = *c as u8;
                if CHAR_BASE <= x && x < (CHAR_BASE + 8) {
                    x -= CHAR_BASE;
                }
                else {
                    return None;
                }
            },
            None => return None
        }

        let y: u8;
        match &value.chars().nth(1) {
            Some(c) => {
                match c.to_digit(10) {
                    Some(d) => {
                        if 1 <= d && d < 9 {
                            y = (d - 1) as u8;
                        }
                        else {
                            return None;
                        }

                    },
                    None => return None
                }
            },
            None => return None
        }

        Some(Square::from_xy(x, y))
    }

    fn from_xy(x: u8, y: u8) -> u8 {
        return (y << 3) | x;
    }

    fn get_fen(self) -> String {
        let rank = (CHAR_BASE + (self & MASK)) as char;
        let file = (self >> 3) + 1;
        format!("{}{}", rank, file)
    }
}
