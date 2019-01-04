use std::fmt;

use crate::global;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;

use crate::move_::Move_;
use crate::piecetype::PieceType;
use crate::square;
use crate::square::Square;

#[derive(Clone, Copy)]
pub struct Position {
    board: [i8; 64],
    active_color: u8,
    castling_status: [bool; 4],
    enpassant_square: Option<Square>,
    halfmoveclock: u32,
    fullmovenumber: u32,
    pieces: [BoardPiece; 32]
}

#[derive(Clone, Copy)]
struct BoardPiece {
    active: bool,
    piece_type: PieceType,
    square: Square
}

impl BoardPiece {
    fn new(active: bool, piece_type: PieceType, square: Square) -> BoardPiece {
        BoardPiece {
            active: active,
            piece_type: piece_type,
            square: square
        }
    }
}

impl Position {
    pub fn new() -> Position {
        Position {
            board: [-1; 64],
            active_color: 0,
            castling_status: [true; 4],
            enpassant_square: None,
            halfmoveclock: 0,
            fullmovenumber: 0,
            pieces: Position::get_new_pieces()
        }
    }

    fn get_new_pieces() -> [BoardPiece; 32] {
        let pieces = [
            BoardPiece::new(false, PieceType::new_rook(COLOR_WHITE), square::A1), //0
            BoardPiece::new(false, PieceType::new_knight(COLOR_WHITE), square::B1),
            BoardPiece::new(false, PieceType::new_bishop(COLOR_WHITE), square::C1),
            BoardPiece::new(false, PieceType::new_queen(COLOR_WHITE), square::D1),
            BoardPiece::new(false, PieceType::new_king(COLOR_WHITE), square::E1),
            BoardPiece::new(false, PieceType::new_bishop(COLOR_WHITE), square::F1),
            BoardPiece::new(false, PieceType::new_knight(COLOR_WHITE), square::G1),
            BoardPiece::new(false, PieceType::new_rook(COLOR_WHITE), square::H1),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), square::A2), //8
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), square::B2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), square::C2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), square::D2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), square::E2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), square::F2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), square::G2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), square::H2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), square::A6), //16
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), square::B6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), square::C6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), square::D6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), square::E6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), square::F6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), square::G6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), square::H6),
            BoardPiece::new(false, PieceType::new_rook(COLOR_BLACK), square::A7), //24
            BoardPiece::new(false, PieceType::new_knight(COLOR_BLACK), square::B7),
            BoardPiece::new(false, PieceType::new_bishop(COLOR_BLACK), square::C7),
            BoardPiece::new(false, PieceType::new_queen(COLOR_BLACK), square::D7),
            BoardPiece::new(false, PieceType::new_king(COLOR_BLACK), square::E7),
            BoardPiece::new(false, PieceType::new_bishop(COLOR_BLACK), square::F7),
            BoardPiece::new(false, PieceType::new_knight(COLOR_BLACK), square::G7),
            BoardPiece::new(false, PieceType::new_rook(COLOR_BLACK), square::H7),
        ];

        pieces
    }

    #[allow(dead_code)]
    pub fn get_board(&self) -> [i8; 64] {
        self.board
    }

    pub fn set_piece(&mut self, square: Square, piece_type: PieceType) -> bool {
        for i in 0..self.pieces.len() {
            if !self.pieces[i].active && self.pieces[i].piece_type == piece_type {
                self.board[square.to_usize()] = i as i8;
                self.pieces[i].active = true;
                self.pieces[i].square = square;
                return true;
            }
        }

        false
    }
    
    pub fn remove_piece(&mut self, square: Square) {
        let i = self.board[square.to_usize()];
        self.pieces[i as usize].active = false;
        self.board[square.to_usize()] = -1;
    }

    pub fn get_piece(&self, square: Square) -> Option<PieceType> {
        let i = self.board[square.to_usize()];
        if i == -1 {
            return None;
        }
        Some(self.pieces[i as usize].piece_type)
    }

    pub fn get_king_square(&self, color: u8) -> Option<Square> {
        let i = match color {
            global::COLOR_WHITE => 4,
            _ => 28
        };

        if self.pieces[i].active {
            return Some(self.pieces[i].square);
        }

        None
    }

    pub fn get_active_pieces(&self) -> Vec<(PieceType, Square)> {
        let mut result: Vec<(PieceType, Square)> = Vec::new();
        let start_index = (self.active_color << 4) as usize;
        for i in start_index..start_index + 16 {
            let p = self.pieces[i];
            if p.active {
                result.push((p.piece_type, p.square));
            }
        }
        result
    }

    pub fn get_all_active_pieces(&self) -> Vec<(PieceType, Square)> {
        let mut result: Vec<(PieceType, Square)> = Vec::new();
        for i in 0..32 {
            let p = self.pieces[i];
            if p.active {
                result.push((p.piece_type, p.square));
            }
        }
        result
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

    fn apply_simple_move(&mut self, square_from: Square, square_to: Square) {
        let piece_index = self.board[square_from.to_usize()];
        self.board[square_to.to_usize()] = piece_index;
        self.pieces[piece_index as usize].square = square_to;
        self.board[square_from.to_usize()] = -1;
    }

    pub fn apply_move(&mut self, mv: u32) {
        //NOTE all moves should be checked at this point
        let move_ = Move_::new(mv);
        let (square_from, square_to) = move_.get_squares();

        let piece_index_from = self.board[square_from.to_usize()];
        let piece = self.get_piece(square_from).unwrap();

        let mut capture = false;
        if self.board[square_to.to_usize()] != -1 {
            capture = true;
            self.remove_piece(square_to);
        }

        self.apply_simple_move(square_from, square_to);

        //en-passant square is filled, pawn moves to it -> en-passant
        //pawn on square in front of en-passant square gets captured
        if move_.is_enpassant() {
            let (x_cap, _) = square_to.to_xy(); // captured pawn has same file as ep square
            let (_, y_cap) = square_from.to_xy(); // captured pawn has same rank as capturing pawn start pos
            self.remove_piece(Square::from_xy(x_cap, y_cap));
            capture = true;
        }

        //promo piece only has type info, not color info
        if move_.is_promotion() {
            let mut promo_piece = move_.get_promo_piece();
            promo_piece.set_color(self.get_active_color());
            self.pieces[piece_index_from as usize].piece_type = promo_piece;
        }

        //castling
        let mut castled = false;
        if move_.is_castling() {
            //e1c1
            if (square_from, square_to) == (square::E1, square::C1) {
                self.apply_simple_move(square::A1, square::D1);
                castled = true;
                self.castling_status[0] = false;
                self.castling_status[1] = false;
            }
            //e1g1
            else if (square_from, square_to) == (square::E1, square::G1) {
                self.apply_simple_move(square::H1, square::F1);
                castled = true;
                self.castling_status[0] = false;
                self.castling_status[1] = false;
            }
            //e8c8
            else if (square_from, square_to) == (square::E8, square::C8) {
                self.apply_simple_move(square::A8, square::D8);
                castled = true;
                self.castling_status[2] = false;
                self.castling_status[3] = false;
            }
            //e8g8
            else if (square_from, square_to) == (square::E8, square::G8) {
                self.apply_simple_move(square::H8, square::F8);
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
        if capture || piece.is_pawn() {
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

    pub fn analyze_move(&self, mv: u32) -> u32 {
        //find out if ep, or castling
        //promotion is already set during parsing
        let mut move_ = Move_::new(mv);
        move_.set_enpassant(false);
        move_.set_castling(false);

        let (square_from, square_to) = move_.get_squares();
        let piece = self.get_piece(square_from).unwrap();

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
        let mut res = "-----------------\n".to_string();
        for y in (0u8..8).rev() {
            for x in 0u8..8 {
                let piece = self.get_piece(Square::from_xy(x, y));
                let c = match piece {
                    Some(p) => p.to_char(),
                    None => ' '
                };
                res.push_str(&format!("|{}", c));
            }
            res.push_str("|\n");
            res.push_str("-----------------\n");
        }
        write!(f, "{}", res)
    }
}