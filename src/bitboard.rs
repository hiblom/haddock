use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::Not;

use std::fmt;

use crate::square::Square;

const LEFT_SIDE: u64 = 0x0101010101010101;
const RIGHT_SIDE: u64 = 0x8080808080808080;

#[derive(Clone, Copy, Eq, Hash)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard(0)
    }

    pub fn from_square(square: Square) -> BitBoard {
        BitBoard(1u64 << square.to_u8())
    }

    pub fn set(&mut self, square: Square) {
        self.0 |= 1u64 << square.to_u8();
    }

    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1u64 << square.to_u8());
    }

    pub fn check(self, square: Square) -> bool {
        self.0 & (1u64 << square.to_u8()) != 0
    }

    pub fn not_empty(self) -> bool {
        return self.0 != 0;
    }

    pub fn up(&mut self) -> Self {
        self.0 <<= 8;
        *self
    }

    pub fn down(&mut self) -> Self {
        self.0 >>= 8;
        *self
    }

    pub fn left(&mut self) -> Self {
        self.0 = (self.0 >> 1) & !RIGHT_SIDE;
        *self
    }

    pub fn right(&mut self) -> Self {
        self.0 = (self.0 << 1) & !LEFT_SIDE;
        *self
    }

    pub fn get_lowest_square(self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }

    pub fn get_highest_square(self) -> Square {
        Square::new(63 - self.0.leading_zeros() as u8)
    }

    pub fn get_square(self) -> Square {
        Square::new(self.0.trailing_zeros() as u8)
    }

    #[allow(dead_code)]
    pub fn get_count(self) -> u32 {
        self.0.count_ones()
    }

    pub fn get_squares(self) -> Vec<Square> {
        let mut x = self.0;
        let n = x.count_ones();
        let mut result: Vec<Square> = Vec::with_capacity(n as usize);
        
        for _ in 0..n {
            let sq = x.trailing_zeros();
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
        let mut res = "".to_string();
        for y in (0u8..8).rev() {
            for x in 0u8..8 {
                let bit = self.0 & (1u64 << (y * 8 + x)) != 0;
                if bit {
                    res.push('X');
                } else {
                    res.push('.');
                }
            }
            res.push_str("\n");
        }
        write!(f, "{}", res)
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}
