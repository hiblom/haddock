use std::fmt;

use crate::global;
use crate::move_::Move_;
use crate::piece::Piece;
use crate::square::Square;
use crate::square::SquareFactory;

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

    pub fn apply_move(&mut self, mv: u32) {
        //NOTE all moves are checked at this point
        let move_ = Move_::new(mv);

        let (square_from, square_to) = move_.get_squares();

        let piece = Piece::new(self.pieces[square_from as usize]);

        let mut capture = self.pieces[square_to as usize] != 0;

        //en-passant square is filled, pawn moves to it -> en-passant
        //pawn on square in front of en-passant square gets captured
        match self.enpassant_square {
            None => (),
            Some(ep_sq) => {
                if square_to == ep_sq && piece.get_type() == global::PIECE_PAWN {
                    let (x_cap, _) = Square::get_xy(square_to); // captured pawn has same file as ep square
                    let (_, y_cap) = Square::get_xy(square_from); // captured pawn has same rank as capturing pawn start pos
                    self.pieces[SquareFactory::create(x_cap, y_cap) as usize] = 0;
                    capture = true;
                }
            }
        }

        //promo piece only has type info, not color info
        if move_.is_promotion() {
            self.pieces[square_to as usize] = move_.get_promo_piece() | self.active_color;
        }
        else {
            self.pieces[square_to as usize] = self.pieces[square_from as usize];
        }
        self.pieces[square_from as usize] = 0;

        //castling
        let mut castled = false;
        if piece.get_type() == global::PIECE_KING {
            //e1c1
            if (square_from, square_to) == (global::E1, global::C1) {
                self.pieces[global::D1 as usize] = self.pieces[global::A1 as usize];
                self.pieces[global::A1 as usize] = 0;
                castled = true;
                self.castling_status[0] = false;
                self.castling_status[1] = false;
            }
            //e1g1
            else if (square_from, square_to) == (global::E1, global::G1) {
                self.pieces[global::F1 as usize] = self.pieces[global::H1 as usize];
                self.pieces[global::H1 as usize] = 0;
                castled = true;
                self.castling_status[0] = false;
                self.castling_status[1] = false;
            }
            //e8c8
            else if (square_from, square_to) == (global::E8, global::C8) {
                self.pieces[global::D8 as usize] = self.pieces[global::A8 as usize];
                self.pieces[global::A8 as usize] = 0;
                castled = true;
                self.castling_status[2] = false;
                self.castling_status[3] = false;
            }
            //e8g8
            else if (square_from, square_to) == (global::E8, global::G8) {
                self.pieces[global::F8 as usize] = self.pieces[global::H8 as usize];
                self.pieces[global::H8 as usize] = 0;
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

        if piece.get_type() == global::PIECE_PAWN {
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
        if capture || piece.get_type() == global::PIECE_PAWN {
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
                let p = self.pieces[SquareFactory::create(x, y) as usize];
                res.push_str(&format!("|{}", Piece::to_char(p)));
            }
            res.push_str("|\n");
            res.push_str("-----------------\n");
        }
        write!(f, "{}", res)
    }
}