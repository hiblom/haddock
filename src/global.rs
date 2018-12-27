pub const FEN_STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const MAX_HALFMOVECLOCK: u32 = 50;


pub const COLOR_WHITE: u8 = 0;
pub const COLOR_BLACK: u8 = 1;

pub const PIECE_KING: u8 = 2;
pub const PIECE_QUEEN: u8 = 4;
pub const PIECE_ROOK: u8 = 6;
pub const PIECE_BISHOP: u8 = 8;
pub const PIECE_KNIGHT: u8 = 10;
pub const PIECE_PAWN: u8 = 12;

pub const PIECE_PAWN_WHITE: u8 = 12;
pub const PIECE_PAWN_BLACK: u8 = 13;


#[allow(dead_code)]
pub const A1:u8 = 00;
#[allow(dead_code)]
pub const A2:u8 = 08;
#[allow(dead_code)]
pub const A3:u8 = 16;
#[allow(dead_code)]
pub const A4:u8 = 24;
#[allow(dead_code)]
pub const A5:u8 = 32;
#[allow(dead_code)]
pub const A6:u8 = 40;
#[allow(dead_code)]
pub const A7:u8 = 48;
#[allow(dead_code)]
pub const A8:u8 = 56;

#[allow(dead_code)]
pub const B1:u8 = 01;
#[allow(dead_code)]
pub const B2:u8 = 09;
#[allow(dead_code)]
pub const B3:u8 = 17;
#[allow(dead_code)]
pub const B4:u8 = 25;
#[allow(dead_code)]
pub const B5:u8 = 33;
#[allow(dead_code)]
pub const B6:u8 = 41;
#[allow(dead_code)]
pub const B7:u8 = 49;
#[allow(dead_code)]
pub const B8:u8 = 57;

#[allow(dead_code)]
pub const C1:u8 = 02;
#[allow(dead_code)]
pub const C2:u8 = 10;
#[allow(dead_code)]
pub const C3:u8 = 18;
#[allow(dead_code)]
pub const C4:u8 = 26;
#[allow(dead_code)]
pub const C5:u8 = 34;
#[allow(dead_code)]
pub const C6:u8 = 42;
#[allow(dead_code)]
pub const C7:u8 = 50;
#[allow(dead_code)]
pub const C8:u8 = 58;

#[allow(dead_code)]
pub const D1:u8 = 03;
#[allow(dead_code)]
pub const D2:u8 = 11;
#[allow(dead_code)]
pub const D3:u8 = 19;
#[allow(dead_code)]
pub const D4:u8 = 27;
#[allow(dead_code)]
pub const D5:u8 = 35;
#[allow(dead_code)]
pub const D6:u8 = 43;
#[allow(dead_code)]
pub const D7:u8 = 51;
#[allow(dead_code)]
pub const D8:u8 = 59;

#[allow(dead_code)]
pub const E1:u8 = 04;
#[allow(dead_code)]
pub const E2:u8 = 12;
#[allow(dead_code)]
pub const E3:u8 = 20;
#[allow(dead_code)]
pub const E4:u8 = 28;
#[allow(dead_code)]
pub const E5:u8 = 36;
#[allow(dead_code)]
pub const E6:u8 = 44;
#[allow(dead_code)]
pub const E7:u8 = 52;
#[allow(dead_code)]
pub const E8:u8 = 60;

#[allow(dead_code)]
pub const F1:u8 = 05;
#[allow(dead_code)]
pub const F2:u8 = 13;
#[allow(dead_code)]
pub const F3:u8 = 21;
#[allow(dead_code)]
pub const F4:u8 = 29;
#[allow(dead_code)]
pub const F5:u8 = 37;
#[allow(dead_code)]
pub const F6:u8 = 45;
#[allow(dead_code)]
pub const F7:u8 = 53;
#[allow(dead_code)]
pub const F8:u8 = 61;

#[allow(dead_code)]
pub const G1:u8 = 06;
#[allow(dead_code)]
pub const G2:u8 = 14;
#[allow(dead_code)]
pub const G3:u8 = 22;
#[allow(dead_code)]
pub const G4:u8 = 30;
#[allow(dead_code)]
pub const G5:u8 = 38;
#[allow(dead_code)]
pub const G6:u8 = 46;
#[allow(dead_code)]
pub const G7:u8 = 54;
#[allow(dead_code)]
pub const G8:u8 = 62;

#[allow(dead_code)]
pub const H1:u8 = 07;
#[allow(dead_code)]
pub const H2:u8 = 15;
#[allow(dead_code)]
pub const H3:u8 = 23;
#[allow(dead_code)]
pub const H4:u8 = 31;
#[allow(dead_code)]
pub const H5:u8 = 39;
#[allow(dead_code)]
pub const H6:u8 = 47;
#[allow(dead_code)]
pub const H7:u8 = 55;
#[allow(dead_code)]
pub const H8:u8 = 63;