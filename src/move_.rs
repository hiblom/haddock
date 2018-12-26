use std::marker;
use crate::square::Square;
use crate::piece::Piece;

const MOVE_MASK_TO: u32           = 0b00000000_00000000_00000000_11111111;
const MOVE_MASK_FROM: u32         = 0b00000000_00000000_11111111_00000000;
const MOVE_MASK_PROMO: u32        = 0b00000000_00000001_00000000_00000000;
const MOVE_MASK_PROMO_PIECE: u32  = 0b11111111_00000000_00000000_00000000;

pub trait MoveFactory {
    fn create(square_from: Self, square_to: Self) -> u32;
}

impl MoveFactory for u8 {
    fn create(square_from: u8, square_to: u8) -> u32 {
        ((square_from as u32) << 8) | (square_to as u32)
    }
}

pub trait Move_ {
    fn new(value: Self) -> Self;
    fn from_str(value: &str) -> Option<Self> where Self: marker::Sized;
    fn get_fen(self) -> String;
    fn get_squares(self) -> (u8, u8);
    fn is_promotion(self) -> bool;
    fn get_promo_piece(self) -> u8;
    fn create_promo_copy(self, picee: u8) -> Self;
}

impl Move_ for u32 {
    fn new(value: u32) -> u32 {
        value
    }

    fn from_str(value: &str) -> Option<u32> {
        let mut result: u32 = 0;
        let len = value.len();

        if  len != 4 && len != 5 {
            return None
        }

        if len == 5 {
            //must be promotion; fifth char contains the piece
            match value.chars().nth(4) {
                Some(c) => {
                    match Piece::from_char(c.to_ascii_uppercase()) {
                        Some::<u8>(p) => {
                            result |= MOVE_MASK_PROMO;
                            result |= (p as u32) << 24;
                        },
                        None => return None
                    }
                },
                None => return None
            }
        }

        let sq_from_str: String = value.chars().take(2).collect();
        match Square::from_str(&sq_from_str) {
            Some::<u8>(s) => result |= (s as u32) << 8,
            None => return None
        }

        let sq_to_str: String = value.chars().skip(2).take(2).collect();
        match Square::from_str(&sq_to_str) {
            Some::<u8>(s) => result |= s as u32,
            None => return None
        }

        Some(result)
    }

    fn get_fen(self) -> String {
        let sq_from: u8 = ((self & MOVE_MASK_FROM) >> 8) as u8;
        let sq_to: u8 = (self & MOVE_MASK_TO) as u8;

        let sq_from_str = Square::get_fen(sq_from);
        let sq_to_str = Square::get_fen(sq_to);

        if self & MOVE_MASK_PROMO == 0 {
            //normal move
            return format!("{}{}", sq_from_str, sq_to_str)
        }

        //promotion move
        let promo_piece: u8 = ((self & MOVE_MASK_PROMO_PIECE) >> 24) as u8;
        let promo_piece_char = Piece::to_char(promo_piece).to_ascii_lowercase();

        format!("{}{}{}", sq_from_str, sq_to_str, promo_piece_char)
    }

    fn get_squares(self) -> (u8, u8) {
        let square_from = ((self & MOVE_MASK_FROM) >> 8) as u8;
        let square_to = (self & MOVE_MASK_TO) as u8;

        (square_from, square_to)
    }

    fn is_promotion(self) -> bool {
        return self & MOVE_MASK_PROMO != 0;
    }

    fn get_promo_piece(self) -> u8 {
        return ((self & MOVE_MASK_PROMO_PIECE) >> 24) as u8;
    }

    fn create_promo_copy(self, piece: u8) -> u32 {
        let mut result = self | MOVE_MASK_PROMO;
        result = (result & !MOVE_MASK_PROMO_PIECE) | ((piece as u32) << 24);
        result
    }
}