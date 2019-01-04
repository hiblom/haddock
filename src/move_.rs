use std::marker;
use crate::square::Square;
use crate::piecetype::PieceType;

const MOVE_MASK_TO: u32           = 0b00000000_00000000_00000000_11111111;
const MOVE_MASK_FROM: u32         = 0b00000000_00000000_11111111_00000000;
const MOVE_BIT_PROMO: u32         = 0b00000000_00000001_00000000_00000000;
const MOVE_BIT_EP: u32            = 0b00000000_00000010_00000000_00000000;
const MOVE_BIT_CASTLING: u32      = 0b00000000_00000100_00000000_00000000;
const MOVE_MASK_PROMO_PIECE: u32  = 0b11111111_00000000_00000000_00000000;

pub trait Move_ {
    fn new(value: Self) -> Self;
    fn from_squares(square_from: Square, square_to: Square) -> Self;
    fn from_str(value: &str) -> Option<Self> where Self: marker::Sized;
    fn get_fen(self) -> String;
    fn get_squares(self) -> (Square, Square);
    fn is_promotion(self) -> bool;
    fn is_castling(self) -> bool;
    fn is_enpassant(self) -> bool;
    fn set_promotion(&mut self, value: bool);
    fn set_castling(&mut self, value: bool);
    fn set_enpassant(&mut self, value: bool);
    fn get_promo_piece(self) -> PieceType;
    fn create_promo_copy(self, piece: PieceType) -> Self;
}

impl Move_ for u32 {
    fn new(value: u32) -> u32 {
        value
    }

    fn from_squares(square_from: Square, square_to: Square) -> u32 {
        (square_from.to_u32() << 8) | square_to.to_u32()
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
                    match PieceType::from_char(c.to_ascii_uppercase()) {
                        Some(p) => {
                            result |= MOVE_BIT_PROMO;
                            result |= (p.to_u8() as u32) << 24;
                        },
                        None => return None
                    }
                },
                None => return None
            }
        }

        let sq_from_str: String = value.chars().take(2).collect();
        match Square::from_str(&sq_from_str) {
            Some::<Square>(s) => result |= (s.to_u32()) << 8,
            None => return None
        }

        let sq_to_str: String = value.chars().skip(2).take(2).collect();
        match Square::from_str(&sq_to_str) {
            Some::<Square>(s) => result |= s.to_u32(),
            None => return None
        }

        Some(result)
    }

    fn get_fen(self) -> String {
        let (sq_from, sq_to) = self.get_squares();

        let sq_from_str = sq_from.to_fen();
        let sq_to_str = sq_to.to_fen();

        if self & MOVE_BIT_PROMO == 0 {
            //normal move
            return format!("{}{}", sq_from_str, sq_to_str)
        }

        //promotion move
        let promo_piece = PieceType::new(((self & MOVE_MASK_PROMO_PIECE) >> 24) as u8);
        let promo_piece_char = promo_piece.to_char().to_ascii_lowercase();

        format!("{}{}{}", sq_from_str, sq_to_str, promo_piece_char)
    }

    fn get_squares(self) -> (Square, Square) {
        let square_from = Square::new(((self & MOVE_MASK_FROM) >> 8) as u8);
        let square_to = Square::new((self & MOVE_MASK_TO) as u8);

        (square_from, square_to)
    }

    fn is_promotion(self) -> bool {
        return self & MOVE_BIT_PROMO != 0;
    }

    fn is_castling(self) -> bool {
        return self & MOVE_BIT_CASTLING != 0;
    }

    fn is_enpassant(self) -> bool {
        return self & MOVE_BIT_EP != 0;
    }

    fn set_promotion(&mut self, value: bool) {
        if value {
            *self |= MOVE_BIT_PROMO;
        }
        else {
            *self &= !MOVE_BIT_PROMO;
        }
    }

    fn set_castling(&mut self, value: bool) {
        if value {
            *self |= MOVE_BIT_CASTLING;
        }
        else {
            *self &= !MOVE_BIT_CASTLING;
        }
    }

    fn set_enpassant(&mut self, value: bool) {
        if value {
            *self |= MOVE_BIT_EP;
        }
        else {
            *self &= !MOVE_BIT_EP;
        }
    }

    fn get_promo_piece(self) -> PieceType {
        PieceType::new(((self & MOVE_MASK_PROMO_PIECE) >> 24) as u8)
    }

    fn create_promo_copy(self, piece: PieceType) -> u32 {
        let mut result = self | MOVE_BIT_PROMO;
        result = (result & !MOVE_MASK_PROMO_PIECE) | ((piece.to_u8() as u32) << 24);
        result
    }
}