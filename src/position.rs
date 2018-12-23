#[derive(Clone, Copy)]
pub struct Position {
    pub pieces: [u8; 64],
    pub active_color: u8,
    pub castling_status: [bool; 4],
    pub enpassant_square: Option<u8>,
    pub halfmoveclock: u32,
    pub fullmovenumber: u32
}

impl Position {
    pub fn new() -> Position {
        Position {
            pieces: [0; 64],
            active_color: 0,
            castling_status: [true; 4],
            enpassant_square: None,
            halfmoveclock: 0,
            fullmovenumber: 0
        }
    }

    pub fn apply_move(&mut self, mut mv: u16) {
        //move consists of 2 byte parts: square from and square to
        const MASK: u16 = 0b0000_0000_1111_1111;
        
        let square_to = (mv & MASK) as usize;
        mv >>= 8;
        
        let square_from = mv as usize;

        //TODO en-passant, castling and promotion
        self.pieces[square_to] = self.pieces[square_from];
        self.pieces[square_from] = 0;

        //TODO clear (or set) en-passant, update halfmove clock, fullmove nr, castling status, active color
    }
}