use std::fmt;

use crate::global;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;

use crate::move_::Move_;
use crate::piecetype::PieceType;
use crate::square::Square;
use crate::square::SquareFactory;

#[derive(Clone, Copy)]
pub struct Position {
    board: [i8; 64],
    active_color: u8,
    castling_status: [bool; 4],
    enpassant_square: Option<u8>,
    halfmoveclock: u32,
    fullmovenumber: u32,
    pieces: [BoardPiece; 32]
}

#[derive(Clone, Copy)]
struct BoardPiece {
    active: bool,
    piece_type: PieceType,
    square: u8
}

impl BoardPiece {
    fn new(active: bool, piece_type: PieceType, square: u8) -> BoardPiece {
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
            BoardPiece::new(false, PieceType::new_rook(COLOR_WHITE), global::A1), //0
            BoardPiece::new(false, PieceType::new_knight(COLOR_WHITE), global::B1),
            BoardPiece::new(false, PieceType::new_bishop(COLOR_WHITE), global::C1),
            BoardPiece::new(false, PieceType::new_queen(COLOR_WHITE), global::D1),
            BoardPiece::new(false, PieceType::new_king(COLOR_WHITE), global::E1),
            BoardPiece::new(false, PieceType::new_bishop(COLOR_WHITE), global::F1),
            BoardPiece::new(false, PieceType::new_knight(COLOR_WHITE), global::G1),
            BoardPiece::new(false, PieceType::new_rook(COLOR_WHITE), global::H1),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), global::A2), //8
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), global::B2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), global::C2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), global::D2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), global::E2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), global::F2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), global::G2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_WHITE), global::H2),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), global::A6), //16
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), global::B6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), global::C6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), global::D6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), global::E6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), global::F6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), global::G6),
            BoardPiece::new(false, PieceType::new_pawn(COLOR_BLACK), global::H6),
            BoardPiece::new(false, PieceType::new_rook(COLOR_BLACK), global::A7), //24
            BoardPiece::new(false, PieceType::new_knight(COLOR_BLACK), global::B7),
            BoardPiece::new(false, PieceType::new_bishop(COLOR_BLACK), global::C7),
            BoardPiece::new(false, PieceType::new_queen(COLOR_BLACK), global::D7),
            BoardPiece::new(false, PieceType::new_king(COLOR_BLACK), global::E7),
            BoardPiece::new(false, PieceType::new_bishop(COLOR_BLACK), global::F7),
            BoardPiece::new(false, PieceType::new_knight(COLOR_BLACK), global::G7),
            BoardPiece::new(false, PieceType::new_rook(COLOR_BLACK), global::H7),
        ];

        pieces
    }

    #[allow(dead_code)]
    pub fn get_board(&self) -> [i8; 64] {
        self.board
    }

    pub fn set_piece(&mut self, square: u8, piece_type: PieceType) -> bool {
        for i in 0..self.pieces.len() {
            if !self.pieces[i].active && self.pieces[i].piece_type == piece_type {
                self.board[square as usize] = i as i8;
                self.pieces[i].active = true;
                self.pieces[i].square = square;
                return true;
            }
        }

        false
    }
    
    pub fn remove_piece(&mut self, square: u8) {
        let i = self.board[square as usize];
        self.pieces[i as usize].active = false;
        self.board[square as usize] = -1;
    }

    pub fn get_piece(&self, square: u8) -> Option<PieceType> {
        let i = self.board[square as usize];
        if i == -1 {
            return None;
        }
        Some(self.pieces[i as usize].piece_type)
    }

    pub fn get_king_square(&self, color: u8) -> Option<u8> {
        let i = match color {
            global::COLOR_WHITE => 4,
            _ => 28
        };

        if self.pieces[i].active {
            return Some(self.pieces[i].square);
        }

        None
    }

    pub fn get_active_pieces(&self) -> Vec<(PieceType, u8)> {
        let mut result: Vec<(PieceType, u8)> = Vec::new();
        let start_index = (self.active_color << 4) as usize;
        for i in start_index..start_index + 16 {
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

    pub fn set_enpassant_square(&mut self, ep_square: Option<u8>) {
        self.enpassant_square = ep_square;
    }

    pub fn get_enpassant_square(&self) -> Option<u8> {
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


    pub fn apply_move(&mut self, mv: u32) {
        //NOTE all moves are checked at this point
        let move_ = Move_::new(mv);
        let (square_from, square_to) = move_.get_squares();

        let piece_index_from = self.board[square_from as usize];
        let piece = self.get_piece(square_from).unwrap();

        let mut capture = false;
        if self.board[square_to as usize] != -1 {
            capture = true;
            self.remove_piece(square_to);
        }

        //en-passant square is filled, pawn moves to it -> en-passant
        //pawn on square in front of en-passant square gets captured
        if move_.is_enpassant() {
            let (x_cap, _) = Square::get_xy(square_to); // captured pawn has same file as ep square
            let (_, y_cap) = Square::get_xy(square_from); // captured pawn has same rank as capturing pawn start pos
            self.remove_piece(SquareFactory::create(x_cap, y_cap));
            capture = true;
        }

        //promo piece only has type info, not color info
        if move_.is_promotion() {
            self.pieces[piece_index_from as usize].piece_type = PieceType::new(move_.get_promo_piece().to_u8() | self.get_active_color());
        }

        self.board[square_to as usize] = piece_index_from;
        self.pieces[piece_index_from as usize].square = square_to;

        //castling
        let mut castled = false;
        if move_.is_castling() {
            //e1c1
            if (square_from, square_to) == (global::E1, global::C1) {
                self.board[global::D1 as usize] = self.board[global::A1 as usize];
                self.board[global::A1 as usize] = 0;
                castled = true;
                self.castling_status[0] = false;
                self.castling_status[1] = false;
            }
            //e1g1
            else if (square_from, square_to) == (global::E1, global::G1) {
                self.board[global::F1 as usize] = self.board[global::H1 as usize];
                self.board[global::H1 as usize] = 0;
                castled = true;
                self.castling_status[0] = false;
                self.castling_status[1] = false;
            }
            //e8c8
            else if (square_from, square_to) == (global::E8, global::C8) {
                self.board[global::D8 as usize] = self.board[global::A8 as usize];
                self.board[global::A8 as usize] = 0;
                castled = true;
                self.castling_status[2] = false;
                self.castling_status[3] = false;
            }
            //e8g8
            else if (square_from, square_to) == (global::E8, global::G8) {
                self.board[global::F8 as usize] = self.board[global::H8 as usize];
                self.board[global::H8 as usize] = 0;
                castled = true;
                self.castling_status[2] = false;
                self.castling_status[3] = false;
            }
        }

        //clear castling status when rook or king moved, or opponent's rook captured
        if !castled {
            if self.active_color == global::COLOR_WHITE {
                if self.castling_status[0] {
                    if square_from == global::E1 || square_from == global::H1 {
                        self.castling_status[0] = false;
                    }
                }
                if self.castling_status[1] {
                    if square_from == global::E1 || square_from == global::A1 {
                        self.castling_status[1] = false;
                    }
                }
                if self.castling_status[2] {
                    if square_to == global::H8 {
                        self.castling_status[2] = false;
                    }
                }
                if self.castling_status[3] {
                    if square_to == global::A8  {
                        self.castling_status[3] = false;
                    }
                }
            }
            else {
                if self.castling_status[2] {
                    if square_from == global::E8 || square_from == global::H8 {
                        self.castling_status[2] = false;
                    }
                }
                if self.castling_status[3] {
                    if square_from == global::E8 || square_from == global::A8 {
                        self.castling_status[3] = false;
                    }
                }
                if self.castling_status[0] {
                    if square_to == global::H1 {
                        self.castling_status[0] = false;
                    }
                }
                if self.castling_status[1] {
                    if square_to == global::A1  {
                        self.castling_status[1] = false;
                    }
                }
            }
        }

        //set en-passant square
        self.enpassant_square = None;

        if piece.is_pawn() {
            let (x_from, y_from) = Square::get_xy(square_from);
            let (_, y_to) = Square::get_xy(square_to);

            if self.active_color == global::COLOR_WHITE && y_from == 1 && y_to == 3 {
                self.enpassant_square = Some(SquareFactory::create(x_from, 2));
            }
            else if self.active_color == global::COLOR_BLACK && y_from == 6 && y_to == 4 {
                self.enpassant_square = Some(SquareFactory::create(x_from, 5));
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
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = "-----------------\n".to_string();
        for y in (0u8..8).rev() {
            for x in 0u8..8 {
                let piece = self.get_piece(SquareFactory::create(x, y));
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