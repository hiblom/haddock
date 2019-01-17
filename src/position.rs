use std::fmt;

use crate::global;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;

use crate::move_::Move_;
use crate::piecetype::PieceType;
use crate::square;
use crate::square::Square;
use crate::bitboard::BitBoard;

#[derive(Clone, Copy)]
pub struct Position {
    bit_boards: [BitBoard; 12],
    active_color: u8,
    castling_status: [bool; 4],
    enpassant_square: Option<Square>,
    halfmoveclock: u32,
    fullmovenumber: u32,
    was_capture: bool
}

impl Position {
    pub fn new() -> Position {
        Position {
            bit_boards: [BitBoard::new(); 12],
            active_color: 0,
            castling_status: [true; 4],
            enpassant_square: None,
            halfmoveclock: 0,
            fullmovenumber: 0,
            was_capture: false
        }
    }

    pub fn set_piece(&mut self, square: Square, piece_type: PieceType) {
        self.bit_boards[piece_type.to_usize()].set(square);
    }
    
    pub fn remove_piece(&mut self, square: Square, piece_type: PieceType) {
        self.bit_boards[piece_type.to_usize()].clear(square);
    }

    pub fn get_piece(&self, square: Square) -> Option<PieceType> {
        for piece_value in 0usize..12 {
            if self.bit_boards[piece_value].check(square) {
                return Some(PieceType::new(piece_value as u8));
            }
        }
        None
    }

    pub fn get_king_square(&self, color: u8) -> Square {
        let piece = PieceType::new_king(color);
        return self.bit_boards[piece.to_usize()].get_square();
    }

    pub fn get_active_color_pieces(&self) -> Vec<(PieceType, Square)> {
        let mut result: Vec<(PieceType, Square)> = Vec::new();
        for piece_value in (self.active_color..12).step_by(2) {
            let squares = self.bit_boards[piece_value as usize].get_squares();
            for square in squares {
                result.push((PieceType::new(piece_value), square));
            }
            
        }
        result
    }

    pub fn get_all_active_pieces(&self) -> Vec<(PieceType, Square)> {
        let mut result: Vec<(PieceType, Square)> = Vec::new();
        for piece_value in 0..12 {
            let squares = self.bit_boards[piece_value as usize].get_squares();
            for square in squares {
                result.push((PieceType::new(piece_value), square));
            }
            
        }
        result
    }

    #[allow(dead_code)]
    pub fn get_piece_counts(&self) -> Vec<(PieceType, u32)> {
        let mut result: Vec<(PieceType, u32)> = Vec::new();
        for piece_value in 0..12 {
            let count = self.bit_boards[piece_value as usize].get_count();
            result.push((PieceType::new(piece_value), count));
        }
        result
    }

    pub fn get_piece_board(&self, color: u8) -> BitBoard {
        return 
            self.bit_boards[PieceType::new_pawn(color).to_usize()] |
            self.bit_boards[PieceType::new_king(color).to_usize()] |
            self.bit_boards[PieceType::new_queen(color).to_usize()] |
            self.bit_boards[PieceType::new_rook(color).to_usize()] |
            self.bit_boards[PieceType::new_bishop(color).to_usize()] |
            self.bit_boards[PieceType::new_knight(color).to_usize()];
    }

    pub fn get_bit_board(&self, piece: PieceType) -> BitBoard {
        return self.bit_boards[piece.to_usize()];
    }

    pub fn set_active_color(&mut self, color: u8) {
        self.active_color = color;
    }

    pub fn get_active_color(&self) -> u8 {
        self.active_color
    }

    pub fn set_castling_status(&mut self, index: usize, active: bool) {
        self.castling_status[index] = active;
    }

    pub fn get_castling_status(&self, index: usize) -> bool {
        self.castling_status[index]
    }

    #[allow(dead_code)]
    pub fn get_full_castling_status(&self) -> [bool; 4] {
        self.castling_status
    }

    pub fn set_enpassant_square(&mut self, ep_square: Option<Square>) {
        self.enpassant_square = ep_square;
    }

    pub fn get_enpassant_square(&self) -> Option<Square> {
        self.enpassant_square
    }

    pub fn set_halfmoveclock(&mut self, halfmoveclock: u32) {
        self.halfmoveclock = halfmoveclock;
    }

    pub fn get_halfmoveclock(&self) -> u32 {
        self.halfmoveclock
    }

    pub fn set_fullmovenumber(&mut self, fullmovenumber: u32) {
        self.fullmovenumber = fullmovenumber;
    }

    pub fn get_fullmovenumber(&self) -> u32 {
        self.fullmovenumber
    }

    #[allow(dead_code)]
    pub fn was_capture(&self) -> bool {
        self.was_capture
    }

    fn apply_simple_move(&mut self, square_from: Square, square_to: Square, piece_type: PieceType) {
        self.set_piece(square_to, piece_type);
        self.remove_piece(square_from, piece_type);
    }

    pub fn apply_move(&mut self, move_: Move_) {
        let (square_from, square_to) = move_.get_squares();
        self.was_capture = false;

        //let piece_index_from = self.board[square_from.to_usize()];
        let piece;
        match self.get_piece(square_from) {
            Some(p) => piece = p,
            None => panic!("No piece found at square {}", square_from.to_fen())
        }

        match self.get_piece(square_to) {
            Some(p) => {
                self.was_capture = true;
                self.remove_piece(square_to, p);
            },
            None => ()
        }

        self.apply_simple_move(square_from, square_to, piece);

        //en-passant square is filled, pawn moves to it -> en-passant
        //pawn on square in front of en-passant square gets captured
        if move_.is_enpassant() {
            let (x_cap, _) = square_to.to_xy(); // captured pawn has same file as ep square
            let (_, y_cap) = square_from.to_xy(); // captured pawn has same rank as capturing pawn start pos
            
            self.remove_piece(Square::from_xy(x_cap, y_cap), PieceType::new_pawn(1 - self.active_color));
            self.was_capture = true;
        }

        //promo piece only has type info, not color info
        if move_.is_promotion() {
            let mut promo_piece = move_.get_promo_piece();
            promo_piece.set_color(self.get_active_color());
            self.remove_piece(square_to, piece);
            self.set_piece(square_to, promo_piece);
        }

        //castling
        let mut castled = false;
        if move_.is_castling() {
            //e1c1
            if square_to == square::C1 {
                self.apply_simple_move(square::A1, square::D1, PieceType::new_rook(COLOR_WHITE));
                castled = true;
                self.castling_status[0] = false;
                self.castling_status[1] = false;
            }
            //e1g1
            else if square_to ==square::G1 {
                self.apply_simple_move(square::H1, square::F1, PieceType::new_rook(COLOR_WHITE));
                castled = true;
                self.castling_status[0] = false;
                self.castling_status[1] = false;
            }
            //e8c8
            else if square_to == square::C8 {
                self.apply_simple_move(square::A8, square::D8, PieceType::new_rook(COLOR_BLACK));
                castled = true;
                self.castling_status[2] = false;
                self.castling_status[3] = false;
            }
            //e8g8
            else if square_to == square::G8 {
                self.apply_simple_move(square::H8, square::F8, PieceType::new_rook(COLOR_BLACK));
                castled = true;
                self.castling_status[2] = false;
                self.castling_status[3] = false;
            }
        }

        //clear castling status when rook or king moved, or opponent's rook captured
        if !castled {
            if self.active_color == global::COLOR_WHITE {
                if self.castling_status[0] {
                    if square_from == square::E1 || square_from == square::H1 {
                        self.castling_status[0] = false;
                    }
                }
                if self.castling_status[1] {
                    if square_from == square::E1 || square_from == square::A1 {
                        self.castling_status[1] = false;
                    }
                }
                if self.castling_status[2] {
                    if square_to == square::H8 {
                        self.castling_status[2] = false;
                    }
                }
                if self.castling_status[3] {
                    if square_to == square::A8  {
                        self.castling_status[3] = false;
                    }
                }
            }
            else {
                if self.castling_status[2] {
                    if square_from == square::E8 || square_from == square::H8 {
                        self.castling_status[2] = false;
                    }
                }
                if self.castling_status[3] {
                    if square_from == square::E8 || square_from == square::A8 {
                        self.castling_status[3] = false;
                    }
                }
                if self.castling_status[0] {
                    if square_to == square::H1 {
                        self.castling_status[0] = false;
                    }
                }
                if self.castling_status[1] {
                    if square_to == square::A1  {
                        self.castling_status[1] = false;
                    }
                }
            }
        }

        //set en-passant square
        self.enpassant_square = None;

        if piece.is_pawn() {
            let (x_from, y_from) = square_from.to_xy();
            let (_, y_to) = square_to.to_xy();

            if self.active_color == global::COLOR_WHITE && y_from == 1 && y_to == 3 {
                self.enpassant_square = Some(Square::from_xy(x_from, 2));
            }
            else if self.active_color == global::COLOR_BLACK && y_from == 6 && y_to == 4 {
                self.enpassant_square = Some(Square::from_xy(x_from, 5));
            }
        }

        //reset halfmove clock when pawn moves, or when there was a capture, otherwise increase
        if self.was_capture || piece.is_pawn() {
            self.halfmoveclock = 0;
        } else {
            self.halfmoveclock += 1;
        }

        //increase fullmove number when color is black
        if self.active_color == global::COLOR_BLACK {
            self.fullmovenumber += 1;
        }

        //flip color
        self.active_color = 1 - self.active_color;
    }

    pub fn analyze_move(&self, mut move_: Move_) -> Move_ {
        //find out if ep, or castling
        //promotion is already set during parsing
        move_.set_enpassant(false);
        move_.set_castling(false);

        let (square_from, square_to) = move_.get_squares();
        let piece;
        match self.get_piece(square_from) {
            Some(p) => piece = p,
            None => {
                panic!("no piece found at {}", square_from.to_fen())
            }
        }

        //ep?
        if piece.is_pawn() {
            match self.get_enpassant_square() {
                Some(s) => {
                    if s == square_to {
                        move_.set_enpassant(true);
                    }
                }
                None => ()
            }
        }

        //castling?
        if piece.is_king() {
            if
                (square_from == square::E1 && (square_to == square::G1 || square_to == square::C1)) ||
                (square_from == square::E8 && (square_to == square::G8 || square_to == square::C8)) {
                move_.set_castling(true);
            }
        }

        move_
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = "".to_string();
        for y in (0u8..8).rev() {
            res.push_str(&format!("{} ", y + 1));
            for x in 0u8..8 {
                let piece = self.get_piece(Square::from_xy(x, y));
                let c = match piece {
                    Some(p) => p.to_char(),
                    None => '.'
                };
                res.push(c);
            }
            res.push_str("\n");
        }
        res.push_str("  abcdefgh\n");
        write!(f, "{}", res)
    }
}