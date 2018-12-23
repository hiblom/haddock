use std::marker;
use crate::square::Square;

const MASK: u16 = 0b0000_0000_1111_1111;

pub trait Move_ {
    fn new(value: Self) -> Self;
    fn from_str(value: &str) -> Option<Self> where Self: marker::Sized;
    fn get_fen(self) -> String;
}

impl Move_ for u16 {
    fn new(value: u16) -> u16 {
        value
    }

    fn from_str(value: &str) -> Option<u16> {
        if value.len() != 4 {
            return None;
        }

        let sq_from_str: String = value.chars().take(2).collect();
        let sq_to_str: String = value.chars().skip(2).take(2).collect();

        let sq_from: u8;
        match Square::from_str(&sq_from_str) {
            Some(s) => sq_from = s,
            None => return None
        }

        let sq_to: u8;
        match Square::from_str(&sq_to_str) {
            Some(s) => sq_to = s,
            None => return None
        }

        let move_ = ((sq_from as u16) << 8) | (sq_to as u16);
        Some(move_)
    }

    fn get_fen(self) -> String {
        let square_to: u8 = (self & MASK) as u8;
        let square_from: u8 = (self >> 8) as u8;
        format!("{}{}", crate::square::Square::get_fen(square_from), crate::square::Square::get_fen(square_to))
    }
}