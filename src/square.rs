const CHAR_BASE: u8 = 97;
const MASK: u8 = 0b0000_0111;

#[allow(dead_code)]
pub const A1:Square = Square(00);
#[allow(dead_code)]
pub const A2:Square = Square(08);
#[allow(dead_code)]
pub const A3:Square = Square(16);
#[allow(dead_code)]
pub const A4:Square = Square(24);
#[allow(dead_code)]
pub const A5:Square = Square(32);
#[allow(dead_code)]
pub const A6:Square = Square(40);
#[allow(dead_code)]
pub const A7:Square = Square(48);
#[allow(dead_code)]
pub const A8:Square = Square(56);

#[allow(dead_code)]
pub const B1:Square = Square(01);
#[allow(dead_code)]
pub const B2:Square = Square(09);
#[allow(dead_code)]
pub const B3:Square = Square(17);
#[allow(dead_code)]
pub const B4:Square = Square(25);
#[allow(dead_code)]
pub const B5:Square = Square(33);
#[allow(dead_code)]
pub const B6:Square = Square(41);
#[allow(dead_code)]
pub const B7:Square = Square(49);
#[allow(dead_code)]
pub const B8:Square = Square(57);

#[allow(dead_code)]
pub const C1:Square = Square(02);
#[allow(dead_code)]
pub const C2:Square = Square(10);
#[allow(dead_code)]
pub const C3:Square = Square(18);
#[allow(dead_code)]
pub const C4:Square = Square(26);
#[allow(dead_code)]
pub const C5:Square = Square(34);
#[allow(dead_code)]
pub const C6:Square = Square(42);
#[allow(dead_code)]
pub const C7:Square = Square(50);
#[allow(dead_code)]
pub const C8:Square = Square(58);

#[allow(dead_code)]
pub const D1:Square = Square(03);
#[allow(dead_code)]
pub const D2:Square = Square(11);
#[allow(dead_code)]
pub const D3:Square = Square(19);
#[allow(dead_code)]
pub const D4:Square = Square(27);
#[allow(dead_code)]
pub const D5:Square = Square(35);
#[allow(dead_code)]
pub const D6:Square = Square(43);
#[allow(dead_code)]
pub const D7:Square = Square(51);
#[allow(dead_code)]
pub const D8:Square = Square(59);

#[allow(dead_code)]
pub const E1:Square = Square(04);
#[allow(dead_code)]
pub const E2:Square = Square(12);
#[allow(dead_code)]
pub const E3:Square = Square(20);
#[allow(dead_code)]
pub const E4:Square = Square(28);
#[allow(dead_code)]
pub const E5:Square = Square(36);
#[allow(dead_code)]
pub const E6:Square = Square(44);
#[allow(dead_code)]
pub const E7:Square = Square(52);
#[allow(dead_code)]
pub const E8:Square = Square(60);

#[allow(dead_code)]
pub const F1:Square = Square(05);
#[allow(dead_code)]
pub const F2:Square = Square(13);
#[allow(dead_code)]
pub const F3:Square = Square(21);
#[allow(dead_code)]
pub const F4:Square = Square(29);
#[allow(dead_code)]
pub const F5:Square = Square(37);
#[allow(dead_code)]
pub const F6:Square = Square(45);
#[allow(dead_code)]
pub const F7:Square = Square(53);
#[allow(dead_code)]
pub const F8:Square = Square(61);

#[allow(dead_code)]
pub const G1:Square = Square(06);
#[allow(dead_code)]
pub const G2:Square = Square(14);
#[allow(dead_code)]
pub const G3:Square = Square(22);
#[allow(dead_code)]
pub const G4:Square = Square(30);
#[allow(dead_code)]
pub const G5:Square = Square(38);
#[allow(dead_code)]
pub const G6:Square = Square(46);
#[allow(dead_code)]
pub const G7:Square = Square(54);
#[allow(dead_code)]
pub const G8:Square = Square(62);

#[allow(dead_code)]
pub const H1:Square = Square(07);
#[allow(dead_code)]
pub const H2:Square = Square(15);
#[allow(dead_code)]
pub const H3:Square = Square(23);
#[allow(dead_code)]
pub const H4:Square = Square(31);
#[allow(dead_code)]
pub const H5:Square = Square(39);
#[allow(dead_code)]
pub const H6:Square = Square(47);
#[allow(dead_code)]
pub const H7:Square = Square(55);
#[allow(dead_code)]
pub const H8:Square = Square(63);

#[derive(Clone, Copy, Eq, Hash)]
pub struct Square(u8);

impl Square {
    pub fn new(value: u8) -> Square {
        Square(value)
    }

    pub fn from_xy(x: u8, y: u8) -> Square {
        Square((y << 3) | x)
    }

    pub fn from_str(value: &str) -> Option<Square> {
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

    pub fn to_fen(self) -> String {
        let rank = (CHAR_BASE + (self.0 & MASK)) as char;
        let file = (self.0 >> 3) + 1;
        format!("{}{}", rank, file)
    }

    pub fn to_xy(self) -> (u8, u8) {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        (x, y)
    }

    pub fn to_u8(self) -> u8 {
        self.0
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    pub fn to_u32(self) -> u32 {
        self.0 as u32
    }

    pub fn up(self) -> Option<Square> {
        let y = self.0 >> 3;
        if y < 7 {
            return Some(Square::new(self.0 + 8));
        }
        None
    }

    pub fn down(self) -> Option<Square> {
        let y = self.0 >> 3;
        if y > 0 {
            return Some(Square::new(self.0 - 8));
        }
        None
    }

    #[allow(dead_code)]
    pub fn left(self) -> Option<Square> {
        let x = self.0 & MASK;
        if x > 0 {
            return Some(Square::new(self.0 - 1));
        }
        None
    }

    #[allow(dead_code)]
    pub fn right(self) -> Option<Square> {
        let x = self.0 & MASK;
        if x < 7 {
            return Some(Square::new(self.0 + 1));
        }
        None
    }

    #[allow(dead_code)]
    pub fn up_left(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x > 0 && y < 7 {
            return Some(Square::new(self.0 + 7));
        }
        None
    }

    #[allow(dead_code)]
    pub fn up_right(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x < 7 && y < 7 {
            return Some(Square::new(self.0 + 9))
        }
        None
    }

    #[allow(dead_code)]
    pub fn down_left(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x > 0 && y > 0 {
            return Some(Square::new(self.0 - 9));
        }
        None
    }

    #[allow(dead_code)]
    pub fn down_right(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x < 7 && y > 0 {
            return Some(Square::new(self.0 - 7));
        }
        None
    }

    #[allow(dead_code)]
    pub fn up_up_right(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x < 7 && y < 6 {
            return Some(Square::new(self.0 + 17));
        }
        None
    }

    #[allow(dead_code)]
    pub fn up_up_left(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x > 0 && y < 6 {
            return Some(Square::new(self.0 + 15));
        }
        None
    }

    #[allow(dead_code)]
    pub fn down_down_right(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x < 7 && y > 1 {
            return Some(Square::new(self.0 - 15));
        }
        None
    }

    #[allow(dead_code)]
    pub fn down_down_left(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x > 0 && y > 1 {
            return Some(Square::new(self.0 - 17));
        }
        None
    }

    #[allow(dead_code)]
    pub fn up_right_right(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x < 6 && y < 7 {
            return Some(Square::new(self.0 + 10));
        }
        None
    }

    #[allow(dead_code)]
    pub fn up_left_left(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x > 1 && y < 7 {
            return Some(Square::new(self.0 + 6));
        }
        None
    }

    #[allow(dead_code)]
    pub fn down_right_right(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x < 6 && y > 0 {
            return Some(Square::new(self.0 - 6));
        }
        None
    }

    #[allow(dead_code)]
    pub fn down_left_left(self) -> Option<Square> {
        let x = self.0 & MASK;
        let y = self.0 >> 3;
        if x > 1 && y > 0 {
            return Some(Square::new(self.0 - 10));
        }
        None
    }

}

impl PartialEq for Square {
    fn eq(&self, other: &Square) -> bool {
        self.0 == other.0
    }
}