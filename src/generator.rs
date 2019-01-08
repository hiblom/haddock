use crate::position::Position;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;
use crate::piecetype;
use crate::piecetype::PieceType;
use crate::move_::Move_;
use crate::square;
use crate::square::Square;
use crate::moveboard;
use crate::bitboard::BitBoard;

const PAWN_PUSH_MOVEBOARD: [usize; 2] = [moveboard::MOVEBOARD_WHITE_PAWN_PUSH, moveboard::MOVEBOARD_BLACK_PAWN_PUSH];
const PAWN_CAP_MOVEBOARD: [usize; 2] = [moveboard::MOVEBOARD_WHITE_PAWN_CAP, moveboard::MOVEBOARD_BLACK_PAWN_CAP];

pub struct Generator<'a> {
    position: &'a Position,
    own_piece_board: BitBoard,
    opp_piece_board: BitBoard,
    all_piece_board: BitBoard
}

impl<'a> Generator<'a> {
    pub fn new(position: &'a Position) -> Generator {
        let own_piece_board = position.get_piece_board(position.get_active_color());
        let opp_piece_board = position.get_piece_board(1 - position.get_active_color());
        Generator { 
            position: position,
            own_piece_board: own_piece_board,
            opp_piece_board: opp_piece_board,
            all_piece_board: own_piece_board | opp_piece_board
        }
    }

    pub fn is_legal_move(&self, move_: Move_) -> bool {
        let color = self.position.get_active_color();
        //find all moves
        let moves = self.generate_moves();
        if moves.contains(&move_) {
            //check castling
            if move_.is_castling() && !self.is_castling_legal(move_) {
                return false;
            }

            let mut pos = self.position.clone();
            pos.apply_move(move_);
            return !Generator::new(&pos).is_check(color);
        }
        false
    }

    pub fn is_castling_legal(&self, move_: Move_) -> bool {
        let color = self.position.get_active_color();

        //cannot castle out of check
        if self.is_check(color) {
            return false;
        }

        //check square that is crossed by king
        let (_, square_to) = move_.get_squares();
        if square_to == square::G1 {
            return !self.is_square_attacked(square::F1, color);
        }
        else if square_to == square::C1 {
            return !self.is_square_attacked(square::D1, color);
        }
        else if square_to == square::G8 {
            return !self.is_square_attacked(square::F8, color);
        }
        else if square_to == square::C8 {
            return !self.is_square_attacked(square::D8, color);
        }
        false //should never happen
    }

    pub fn generate_moves(&self) -> Vec<Move_> {
        let mut result: Vec<Move_> = Vec::new();

        for (piece_type, square) in self.position.get_active_color_pieces() {
            let mut piece_moves = self.generate_piece_moves(square, piece_type);
            result.append(&mut piece_moves);
        }

        result
    }

    pub fn generate_piece_moves(&self, square: Square, piece_type: PieceType) -> Vec<Move_> {

        let pt = piece_type.get_type();
        match pt {
            piecetype::PIECE_KING => self.generate_king_moves(square),
            piecetype::PIECE_QUEEN => self.generate_queen_moves(square),
            piecetype::PIECE_ROOK => self.generate_rook_moves(square),
            piecetype::PIECE_BISHOP => self.generate_bishop_moves(square),
            piecetype::PIECE_KNIGHT => self.generate_moveboard_moves(square, moveboard::MOVEBOARD_KNIGHT),
            piecetype::PIECE_PAWN => self.generate_pawn_moves(square),
            _ => Vec::new()
        }
    }

     fn generate_moveboard_moves(&self, current_square: Square, mb: usize) -> Vec<Move_> {
        let mut result: Vec<Move_> = Vec::new();

        let mut move_board = moveboard::get_move_board(mb, current_square);
        move_board &= !self.own_piece_board; //exclude moves to pieces of same color

        for sq in move_board.get_squares() {
            result.push(Move_::from_squares(current_square, sq));
        }
        
        result
    }

    fn generate_rook_moves(&self, current_square: Square) -> Vec<Move_> {
        let mut result: Vec<Move_> = Vec::new();

        let mut move_board = BitBoard::new();
        move_board |= self.generate_move_board(moveboard::DIR_UP, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_RIGHT, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_LEFT, current_square, BitBoard::get_highest_square);
        
        for sq in move_board.get_squares() {
            result.push(Move_::from_squares(current_square, sq));
        }

        result
    }

    fn generate_bishop_moves(&self, current_square: Square) -> Vec<Move_> {
        let mut result: Vec<Move_> = Vec::new();

        let mut move_board = BitBoard::new();
        move_board |= self.generate_move_board(moveboard::DIR_UP_RIGHT, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN_RIGHT, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN_LEFT, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_UP_LEFT, current_square, BitBoard::get_lowest_square);

        for sq in move_board.get_squares() {
            result.push(Move_::from_squares(current_square, sq));
        }

        result
    }

    fn generate_queen_moves(&self, current_square: Square) -> Vec<Move_> {
        let mut result: Vec<Move_> = Vec::new();

        let mut move_board = BitBoard::new();
        move_board |= self.generate_move_board(moveboard::DIR_UP, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_RIGHT, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_LEFT, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_UP_RIGHT, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN_RIGHT, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN_LEFT, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_UP_LEFT, current_square, BitBoard::get_lowest_square);

        for sq in move_board.get_squares() {
            result.push(Move_::from_squares(current_square, sq));
        }

        result
    }


    fn generate_move_board(&self, direction: usize, current_square: Square, get_nearest: fn(BitBoard) -> Square) -> BitBoard {
        let mut move_board = BitBoard::new();
        let forward_ray_board = moveboard::get_ray_board(direction, current_square);
        let inter = forward_ray_board & self.all_piece_board;
        if inter.not_empty() {
            let nearest_square = get_nearest(inter);
            let reverse_ray_board = moveboard::get_ray_board((direction + 4) % 8, nearest_square);
            move_board |= forward_ray_board & reverse_ray_board;
            move_board |= BitBoard::from_square(nearest_square) & self.opp_piece_board; //add capture
        } else {
            move_board |= forward_ray_board;
        }
        move_board
    }

    pub fn generate_king_moves(&self, current_square: Square) -> Vec<Move_> {
        let mut result = self.generate_moveboard_moves(current_square, moveboard::MOVEBOARD_KING);

        let color = self.position.get_active_color();
        //castling
        if color == COLOR_WHITE {
            if self.position.get_castling_status(0) {
                //white K-side
                if  self.position.get_piece(square::F1).is_none() && 
                    self.position.get_piece(square::G1).is_none() {
                        let mut mv = Move_::from_squares(square::E1, square::G1);
                        mv.set_castling(true);
                        result.push(mv);
                }
            }
            if self.position.get_castling_status(1) {
                //white Q-side
                if  self.position.get_piece(square::B1).is_none() && 
                    self.position.get_piece(square::C1).is_none() && 
                    self.position.get_piece(square::D1).is_none() {
                        let mut mv = Move_::from_squares(square::E1, square::C1);
                        mv.set_castling(true);
                        result.push(mv);
                }
            }
        }
        else {
            if self.position.get_castling_status(2) {
                //black K-side
                if  self.position.get_piece(square::F8).is_none() && 
                    self.position.get_piece(square::G8).is_none() {
                        let mut mv = Move_::from_squares(square::E8, square::G8);
                        mv.set_castling(true);
                        result.push(mv);
                }
            }
            if self.position.get_castling_status(3) {
                //black Q-side
                if  self.position.get_piece(square::B8).is_none() && 
                    self.position.get_piece(square::C8).is_none() && 
                    self.position.get_piece(square::D8).is_none() {
                        let mut mv = Move_::from_squares(square::E8, square::C8);
                        mv.set_castling(true);
                        result.push(mv);
                }
            }
        }

        result
    }

    pub fn generate_pawn_moves(&self, current_square: Square) -> Vec<Move_> {
        let color = self.position.get_active_color();
        let mut moves: Vec<Move_> = Vec::new();

        let mut move_board = moveboard::get_move_board(PAWN_PUSH_MOVEBOARD[color as usize], current_square);
        move_board &= !self.all_piece_board; //exclude moves to occupied squares
        let pushed = move_board.not_empty();

        let cap_board = moveboard::get_move_board(PAWN_CAP_MOVEBOARD[color as usize], current_square);
        move_board |= cap_board & self.opp_piece_board; //only include captures
        
        for sq in move_board.get_squares() {
            moves.push(Move_::from_squares(current_square, sq));
        }
        
        let (_, current_y) = current_square.to_xy();

        //double move
        if pushed {
            if color == COLOR_WHITE && current_y == 1 {
                let mut sq_to = current_square.up().unwrap();
                sq_to = sq_to.up().unwrap();
                if self.position.get_piece(sq_to).is_none() {
                    moves.push(Move_::from_squares(current_square, sq_to));
                }
            }
            else if color == COLOR_BLACK && current_y == 6 {
                let mut sq_to = current_square.down().unwrap();
                sq_to = sq_to.down().unwrap();
                if self.position.get_piece(sq_to).is_none() {
                    moves.push(Move_::from_squares(current_square, sq_to));
                }
            }
        }

        //en-passant
        match self.position.get_enpassant_square() {
            None => (),
            Some(ep_sq) => {
                let ep_bb = BitBoard::from_square(ep_sq);
                if (ep_bb & cap_board).not_empty() {
                    let mut mv = Move_::from_squares(current_square, ep_sq);
                    mv.set_enpassant(true);
                    moves.push(mv);
                }
            }
        }

        //promotion
        let mut result: Vec<Move_> = Vec::new();

        for m in moves {
            let (_, sq_to) = Move_::get_squares(m);
            let (_, y_to) = sq_to.to_xy();
            if (color == COLOR_WHITE && y_to == 7) || (color == COLOR_BLACK && y_to == 0) {
                result.push(Move_::create_promo_copy(m, PieceType::new_queen(COLOR_WHITE)));
                result.push(Move_::create_promo_copy(m, PieceType::new_rook(COLOR_WHITE)));
                result.push(Move_::create_promo_copy(m, PieceType::new_bishop(COLOR_WHITE)));
                result.push(Move_::create_promo_copy(m, PieceType::new_knight(COLOR_WHITE)));
            }
            else {
                result.push(m);
            }
        }
        result
    }

    pub fn is_check(&self, color: u8) -> bool {
        let s = self.position.get_king_square(color);
        return self.is_square_attacked(s, color);
    }

    pub fn is_square_attacked(&self, square: Square, color: u8) -> bool {
        let other_color = 1 - color;

        //attacked by king?
        let mb = moveboard::get_move_board(moveboard::MOVEBOARD_KING, square);
        let bb = self.position.get_bit_board(PieceType::new_king(other_color));
        if (mb & bb).not_empty() {
            return true;
        }

        //attacked by knight
        let mb = moveboard::get_move_board(moveboard::MOVEBOARD_KNIGHT, square);
        let bb = self.position.get_bit_board(PieceType::new_knight(other_color));
        if (mb & bb).not_empty() {
            return true;
        }

        //attacked by pawn. use own color pawn capture board to intersect with opp pawns
        let mb = moveboard::get_move_board(PAWN_CAP_MOVEBOARD[color as usize], square);
        let bb = self.position.get_bit_board(PieceType::new_pawn(other_color));
        if (mb & bb).not_empty() {
            return true;
        }

        //sliding piece attacks
        return
            self.find_orthogonal_attacker(moveboard::DIR_UP, square, BitBoard::get_lowest_square, other_color) ||
            self.find_orthogonal_attacker(moveboard::DIR_RIGHT, square, BitBoard::get_lowest_square, other_color) ||
            self.find_orthogonal_attacker(moveboard::DIR_DOWN, square, BitBoard::get_highest_square, other_color) ||
            self.find_orthogonal_attacker(moveboard::DIR_LEFT, square, BitBoard::get_highest_square, other_color) ||
            self.find_diagonal_attacker(moveboard::DIR_UP_RIGHT, square, BitBoard::get_lowest_square, other_color) ||
            self.find_diagonal_attacker(moveboard::DIR_DOWN_RIGHT, square, BitBoard::get_highest_square, other_color) ||
            self.find_diagonal_attacker(moveboard::DIR_DOWN_LEFT, square, BitBoard::get_highest_square, other_color) ||
            self.find_diagonal_attacker(moveboard::DIR_UP_LEFT, square, BitBoard::get_lowest_square, other_color);
    }

    fn find_orthogonal_attacker(&self, direction: usize, square: Square, get_nearest: fn(BitBoard) -> Square, other_color: u8) -> bool {
        let forward_ray_board = moveboard::get_ray_board(direction, square);
        let inter = forward_ray_board & self.all_piece_board;
        if inter.not_empty() {
            let nearest_square = get_nearest(inter);
            let bb = 
                self.position.get_bit_board(PieceType::new_queen(other_color)) | 
                self.position.get_bit_board(PieceType::new_rook(other_color));
            
            return (BitBoard::from_square(nearest_square) & bb).not_empty();
        }
        false
    }

    fn find_diagonal_attacker(&self, direction: usize, square: Square, get_nearest: fn(BitBoard) -> Square, other_color: u8) -> bool {
        let forward_ray_board = moveboard::get_ray_board(direction, square);
        let inter = forward_ray_board & self.all_piece_board;
        if inter.not_empty() {
            let nearest_square = get_nearest(inter);
            let bb = 
                self.position.get_bit_board(PieceType::new_queen(other_color)) | 
                self.position.get_bit_board(PieceType::new_bishop(other_color));
            
            return (BitBoard::from_square(nearest_square) & bb).not_empty();
        }
        false
    }
}