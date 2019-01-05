use crate::global;

pub const PIECE_PAWN: u8 = 0;
pub const PIECE_KING: u8 = 2;
pub const PIECE_QUEEN: u8 = 4;
pub const PIECE_ROOK: u8 = 6;
pub const PIECE_BISHOP: u8 = 8;
pub const PIECE_KNIGHT: u8 = 10;

const MASK_COLOR: u8 = 0b0000_0001;
const MASK_TYPE: u8 = 0b1111_1110;

#[derive(Clone, Copy, Eq, Hash)]
pub struct PieceType(u8);

impl PieceType {
    pub fn new(value: u8) -> PieceType  {
        PieceType(value)
    }

    pub fn new_king(color: u8) -> PieceType {
        PieceType(PIECE_KING | color)
    }

    pub fn new_queen(color: u8) -> PieceType {
        PieceType(PIECE_QUEEN | color)
    }

    pub fn new_rook(color: u8) -> PieceType {
        PieceType(PIECE_ROOK | color)
    }

    pub fn new_bishop(color: u8) -> PieceType {
        PieceType(PIECE_BISHOP | color)
    }

    pub fn new_knight(color: u8) -> PieceType {
        PieceType(PIECE_KNIGHT | color)
    }

    pub fn new_pawn(color: u8) -> PieceType {
        PieceType(PIECE_PAWN | color)
    }

    pub fn to_u8(self) -> u8 {
        self.0
    }

    pub fn to_usize(self) -> usize {
        self.0 as usize
    }

    pub fn from_char(value: char) -> Option<PieceType> {
        let piece_value: u8;

        match value.to_ascii_lowercase() {
            'k' => piece_value = PIECE_KING,
            'q' => piece_value = PIECE_QUEEN,
            'r' => piece_value = PIECE_ROOK,
            'b' => piece_value = PIECE_BISHOP,
            'n' => piece_value = PIECE_KNIGHT,
            'p' => piece_value = PIECE_PAWN,
            _ => return None
        }

        let piece_color = value.is_ascii_lowercase() as u8;
        Some(PieceType(piece_value | piece_color))
    }

    pub fn to_char(self) -> char {
        let c = match self.0 & MASK_TYPE {
            PIECE_KING => 'K',
            PIECE_QUEEN => 'Q',
            PIECE_ROOK => 'R',
            PIECE_BISHOP => 'B',
            PIECE_KNIGHT => 'N',
            PIECE_PAWN => 'P',
            _ => ' '
        };

        if self.0 & MASK_COLOR == global::COLOR_BLACK {
            return c.to_ascii_lowercase()
        }
        c
    }

    pub fn get_color(self) -> u8 {
        self.0 & MASK_COLOR
    }

    pub fn set_color(&mut self, color: u8) {
        self.0 = (self.0 & MASK_TYPE) | color;
    }

    pub fn get_type(self) -> u8 {
        self.0 & MASK_TYPE
    }

    pub fn get_move_type(self) -> PieceType {
        let raw_type = self.0 & MASK_TYPE;
        
        if raw_type == PIECE_PAWN {
            return PieceType(self.0);
        }

        PieceType(self.0 & MASK_TYPE)
    }

    pub fn has_color(self, color: u8) -> bool {
        color == self.0 & MASK_COLOR
    }

    pub fn is_pawn(self) -> bool {
         self.0 & MASK_TYPE == PIECE_PAWN
    }

    pub fn is_king(self) -> bool {
         self.0 & MASK_TYPE == PIECE_KING
    }
}

impl PartialEq for PieceType {
    fn eq(&self, other: &PieceType) -> bool {
        self.0 == other.0
    }
}