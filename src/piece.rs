use crate::global;
use std::marker;

const MASK_COLOR: u8 = 0b0000_0001;
const MASK_TYPE: u8 = 0b1111_1110;

pub trait Piece {
    fn new(value: Self) -> Self;
    fn from_char(value: char) -> Option<Self> where Self: marker::Sized;
    fn to_char(self) -> char;
    fn get_color(self) -> u8;
    fn get_type(self) -> u8;
    fn has_color(self, color: u8) -> bool;
}

impl Piece for u8 {
    fn new(value: u8) -> u8 {
        value
    }

    fn from_char(value: char) -> Option<u8> {
        let piece_type: u8;
        if !value.is_ascii() {
            return None
        }

        match value.to_ascii_lowercase() {
            'k' => piece_type = global::PIECE_KING,
            'q' => piece_type = global::PIECE_QUEEN,
            'r' => piece_type = global::PIECE_ROOK,
            'b' => piece_type = global::PIECE_BISHOP,
            'n' => piece_type = global::PIECE_KNIGHT,
            'p' => piece_type = global::PIECE_PAWN,
            _ => return None
        }

        let piece_color = value.is_ascii_lowercase() as u8;
        Some (piece_type | piece_color)
    }

    fn to_char(self) -> char {
        let c = match self & MASK_TYPE {
            global::PIECE_KING => 'K',
            global::PIECE_QUEEN => 'Q',
            global::PIECE_ROOK => 'R',
            global::PIECE_BISHOP => 'B',
            global::PIECE_KNIGHT => 'N',
            global::PIECE_PAWN => 'P',
            _ => ' '
        };

        if self & MASK_COLOR == global::COLOR_BLACK {
            return c.to_ascii_lowercase()
        }
        c
    }

    fn get_color(self) -> u8 {
        self & MASK_COLOR
    }

    fn get_type(self) -> u8 {
        self & MASK_TYPE
    }

    fn has_color(self, color: u8) -> bool {
        color == self & MASK_COLOR
    }
}
