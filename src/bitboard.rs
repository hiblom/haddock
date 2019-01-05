use std::fmt;

use crate::square::Square;

#[derive(Clone, Copy, Eq, Hash)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard(0)
    }

    pub fn set(&mut self, square: Square) {
        self.0 |= 1u64 << square.to_u8();
    }

    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1u64 << square.to_u8());
    }

    pub fn check(self, square: Square) -> bool {
        (self.0 >> square.to_u8()) & 1u64 != 0
    }

    pub fn get_square(self) -> Square {
        Square::new((self.0 - 1).count_ones() as u8)
    }

    pub fn get_squares(self) -> Vec<Square> {
        let mut x = self.0;
        let mut result: Vec<Square> = Vec::new();
        let mut n = x.count_ones();
        while n > 0 {
            n -= 1;
            let sq = (x - 1).count_ones() - n;
            result.push(Square::new(sq as u8));
            x &= !(1u64 << sq);
        }
        result
    }
}

impl PartialEq for BitBoard {
    fn eq(&self, other: &BitBoard) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}