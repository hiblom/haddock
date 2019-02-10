use crate::global::COLOR_WHITE;
use crate::position::Position;
use crate::piecetype;
use crate::piecetype::PieceType;
use crate::move_::Move_;
use crate::square;
use crate::square::Square;
use crate::moveboard;
use crate::bitboard::BitBoard;
use crate::moveresult::MoveResult;
use crate::hash_counter::HashCounter;

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
            position,
            own_piece_board,
            opp_piece_board,
            all_piece_board: own_piece_board | opp_piece_board
        }
    }

    pub fn is_legal_move(&self, move_: Move_) -> bool {
        let color = self.position.get_active_color();
        //find all moves
        let moves = self.generate_moves(false);
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

    pub fn try_apply_move(&self, move_: Move_, history: &HashCounter) -> MoveResult {
        let color = self.position.get_active_color();

        //check castling
        if move_.is_castling() && !self.is_castling_legal(move_) {
            return MoveResult::Illegal;
        }

        let mut pos = self.position.clone();
        pos.apply_move(move_);
        if Generator::new(&pos).is_check(color) {
            return MoveResult::Illegal;
        }

        //draw by halfmove clock
        if pos.is_draw_by_halfmoveclock() {
            return MoveResult::Draw;
        }

        //draw by 3-fold repetition
        if history.get(pos.get_hash()) >= 2 {
            return MoveResult::Draw;
        }

        MoveResult::Next(pos)
    }

    pub fn generate_moves(&self, only_captures: bool) -> Vec<Move_> {
        let mut result: Vec<Move_> = Vec::with_capacity(80);

        for (piece_type, square) in self.position.get_active_color_pieces_no_pawns() {
            self.generate_piece_moves(square, piece_type, only_captures, &mut result);
        }
        self.generate_pawn_moves_2(only_captures, &mut result);
        
        result
    }

    pub fn generate_piece_moves(&self, square: Square, piece_type: PieceType, only_captures: bool, moves: &mut Vec<Move_>) {

        let pt = piece_type.get_type();
        match pt {
            piecetype::PIECE_KING => self.generate_king_moves(square, only_captures,moves),
            piecetype::PIECE_QUEEN => self.generate_queen_moves(square, only_captures, moves),
            piecetype::PIECE_ROOK => self.generate_rook_moves(square, only_captures, moves),
            piecetype::PIECE_BISHOP => self.generate_bishop_moves(square, only_captures, moves),
            piecetype::PIECE_KNIGHT => self.generate_moveboard_moves(square, moveboard::MOVEBOARD_KNIGHT, only_captures, moves),
            _ => ()
        }
    }

    fn generate_moveboard_moves(&self, current_square: Square, mb: usize, only_captures: bool, moves: &mut Vec<Move_>) {
        let mut move_board = moveboard::get_move_board(mb, current_square);
        
        move_board &= !self.own_piece_board; //exclude moves to pieces of same color

        if !only_captures {
            let silent_board = move_board & !self.opp_piece_board;
            for sq in silent_board.get_squares() {
                moves.push(Move_::from_squares(current_square, sq));
            }
        }

        let capture_board = move_board & self.opp_piece_board;
        for sq in capture_board.get_squares() {
            let mut mv = Move_::from_squares(current_square, sq);
            mv.set_capture();
            moves.push(mv);
        }
    }

    fn generate_rook_moves(&self, current_square: Square, only_captures: bool, moves: &mut Vec<Move_>) {
        let mut move_board = BitBoard::new();
        move_board |= self.generate_move_board(moveboard::DIR_UP, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_RIGHT, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_LEFT, current_square, BitBoard::get_highest_square);

        if !only_captures {
            let silent_board = move_board & !self.opp_piece_board;
            for sq in silent_board.get_squares() {
                moves.push(Move_::from_squares(current_square, sq));
            }
        }

        let capture_board = move_board & self.opp_piece_board;
        for sq in capture_board.get_squares() {
            let mut mv = Move_::from_squares(current_square, sq);
            mv.set_capture();
            moves.push(mv);
        }
    }

    fn generate_bishop_moves(&self, current_square: Square, only_captures: bool, moves: &mut Vec<Move_>) {
        let mut move_board = BitBoard::new();
        move_board |= self.generate_move_board(moveboard::DIR_UP_RIGHT, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN_RIGHT, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN_LEFT, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_UP_LEFT, current_square, BitBoard::get_lowest_square);

        if !only_captures {
            let silent_board = move_board & !self.opp_piece_board;
            for sq in silent_board.get_squares() {
                moves.push(Move_::from_squares(current_square, sq));
            }
        }

        let capture_board = move_board & self.opp_piece_board;
        for sq in capture_board.get_squares() {
            let mut mv = Move_::from_squares(current_square, sq);
            mv.set_capture();
            moves.push(mv);
        }
    }

    fn generate_queen_moves(&self, current_square: Square, only_captures: bool, moves: &mut Vec<Move_>) {
        let mut move_board = BitBoard::new();
        move_board |= self.generate_move_board(moveboard::DIR_UP, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_RIGHT, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_LEFT, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_UP_RIGHT, current_square, BitBoard::get_lowest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN_RIGHT, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_DOWN_LEFT, current_square, BitBoard::get_highest_square);
        move_board |= self.generate_move_board(moveboard::DIR_UP_LEFT, current_square, BitBoard::get_lowest_square);

        if !only_captures {
            let silent_board = move_board & !self.opp_piece_board;
            for sq in silent_board.get_squares() {
                moves.push(Move_::from_squares(current_square, sq));
            }
        }

        let capture_board = move_board & self.opp_piece_board;
        for sq in capture_board.get_squares() {
            let mut mv = Move_::from_squares(current_square, sq);
            mv.set_capture();
            moves.push(mv);
        }
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

    pub fn generate_king_moves(&self, current_square: Square, only_captures: bool, moves: &mut Vec<Move_>) {
        self.generate_moveboard_moves(current_square, moveboard::MOVEBOARD_KING, only_captures, moves);

        if !only_captures {
            let color = self.position.get_active_color();
            //castling
            if color == COLOR_WHITE {
                if self.position.get_castling_status(0) {
                    //white K-side
                    if self.position.get_piece(square::F1).is_none() &&
                        self.position.get_piece(square::G1).is_none() {
                        let mut mv = Move_::from_squares(square::E1, square::G1);
                        mv.set_castling();
                        moves.push(mv);
                    }
                }
                if self.position.get_castling_status(1) {
                    //white Q-side
                    if self.position.get_piece(square::B1).is_none() &&
                        self.position.get_piece(square::C1).is_none() &&
                        self.position.get_piece(square::D1).is_none() {
                        let mut mv = Move_::from_squares(square::E1, square::C1);
                        mv.set_castling();
                        moves.push(mv);
                    }
                }
            } else {
                if self.position.get_castling_status(2) {
                    //black K-side
                    if self.position.get_piece(square::F8).is_none() &&
                        self.position.get_piece(square::G8).is_none() {
                        let mut mv = Move_::from_squares(square::E8, square::G8);
                        mv.set_castling();
                        moves.push(mv);
                    }
                }
                if self.position.get_castling_status(3) {
                    //black Q-side
                    if self.position.get_piece(square::B8).is_none() &&
                        self.position.get_piece(square::C8).is_none() &&
                        self.position.get_piece(square::D8).is_none() {
                        let mut mv = Move_::from_squares(square::E8, square::C8);
                        mv.set_castling();
                        moves.push(mv);
                    }
                }
            }
        }
    }

    //generate all pawn moves at once
    pub fn generate_pawn_moves_2(&self, only_captures: bool, moves: &mut Vec<Move_>) {
        let color = self.position.get_active_color();

        let promo_rank: u8;
        let forward: fn(&mut BitBoard) -> BitBoard;
        let back: fn(&mut BitBoard) -> BitBoard;

        match color {
            COLOR_WHITE => {
                promo_rank = 7;
                forward = BitBoard::up;
                back = BitBoard::down;
            }
            _ => {
                promo_rank = 0;
                forward = BitBoard::down;
                back = BitBoard::up;
            }
        };

        let pawn_board = self.position.get_active_color_pawn_board();

        if !only_captures {
            //one square forward
            let dest_board = forward(&mut pawn_board.clone()) & !self.all_piece_board; //must be empty
            let src_board = back(&mut dest_board.clone()); //and back to start pos

            let start_squares = src_board.get_squares();
            let end_squares = dest_board.get_squares();

            for i in 0..start_squares.len() {
                let (_, y_to) = end_squares[i].to_xy();
                let m = Move_::from_squares(start_squares[i], end_squares[i]);

                if y_to == promo_rank {
                    moves.push(Move_::create_promo_copy(m, PieceType::new_queen(COLOR_WHITE)));
                    moves.push(Move_::create_promo_copy(m, PieceType::new_rook(COLOR_WHITE)));
                    moves.push(Move_::create_promo_copy(m, PieceType::new_bishop(COLOR_WHITE)));
                    moves.push(Move_::create_promo_copy(m, PieceType::new_knight(COLOR_WHITE)));
                } else {
                    moves.push(m);
                }
            }

            //two squares forward
            let mut dest_board = pawn_board & BitBoard::pawn_start_ranks_board();
            dest_board = forward(&mut dest_board) & !self.all_piece_board; //must be empty
            dest_board = forward(&mut dest_board) & !self.all_piece_board; //must be empty
            let src_board = back(&mut back(&mut dest_board.clone())); //and back to start pos

            let start_squares = src_board.get_squares();
            let end_squares = dest_board.get_squares();

            for i in 0..start_squares.len() {
                let m = Move_::from_squares(start_squares[i], end_squares[i]);
                moves.push(m);
            }
        }

        //create board from opp board and EP square
        
        let mut ep_board = BitBoard::new();
        match self.position.get_enpassant_square() {
            Some(eq_sq) => ep_board = BitBoard::from_square(eq_sq),
            None => ()
        }
        let cap_board = self.opp_piece_board | ep_board;

        //capture left
        let dest_board = forward(&mut pawn_board.clone().left()) & cap_board; //must have opponent
        let src_board = back(&mut dest_board.clone()).right(); //and back to start pos

        let start_squares = src_board.get_squares();
        let end_squares = dest_board.get_squares();

        for i in 0..start_squares.len() {
            let (_, y_to) = end_squares[i].to_xy();
            let mut m = Move_::from_squares(start_squares[i], end_squares[i]);
            m.set_capture();
            if ep_board.not_empty() && ep_board == BitBoard::from_square(end_squares[i]) {
                m.set_enpassant();
            }

            if y_to == promo_rank {
                moves.push(Move_::create_promo_copy(m, PieceType::new_queen(COLOR_WHITE)));
                moves.push(Move_::create_promo_copy(m, PieceType::new_rook(COLOR_WHITE)));
                moves.push(Move_::create_promo_copy(m, PieceType::new_bishop(COLOR_WHITE)));
                moves.push(Move_::create_promo_copy(m, PieceType::new_knight(COLOR_WHITE)));
            }
            else {
                moves.push(m);
            }
        }

        //capture right
        let dest_board = forward(&mut pawn_board.clone().right()) & cap_board; //must have opponent
        let src_board = back(&mut dest_board.clone()).left(); //and back to start pos

        let start_squares = src_board.get_squares();
        let end_squares = dest_board.get_squares();

        for i in 0..start_squares.len() {
            let (_, y_to) = end_squares[i].to_xy();
            let mut m = Move_::from_squares(start_squares[i], end_squares[i]);
            m.set_capture();
            if ep_board.not_empty() && ep_board == BitBoard::from_square(end_squares[i]) {
                m.set_enpassant();
            }

            if y_to == promo_rank {
                moves.push(Move_::create_promo_copy(m, PieceType::new_queen(COLOR_WHITE)));
                moves.push(Move_::create_promo_copy(m, PieceType::new_rook(COLOR_WHITE)));
                moves.push(Move_::create_promo_copy(m, PieceType::new_bishop(COLOR_WHITE)));
                moves.push(Move_::create_promo_copy(m, PieceType::new_knight(COLOR_WHITE)));
            }
            else {
                moves.push(m);
            }
        }
    }

    pub fn is_check(&self, color: u8) -> bool {
        let s = self.position.get_king_square(color);
        return self.is_square_attacked(s, color);
    }

    #[allow(dead_code)]
    pub fn find_lowest_attacker(&self, square: Square, color: u8) -> Option<Square> {
        let other_color = 1 - color;

        //pawn? use own color pawn capture board to intersect with opp pawns
        let mb = moveboard::get_move_board(PAWN_CAP_MOVEBOARD[color as usize], square);
        let bb = self.position.get_bit_board(PieceType::new_pawn(other_color));
        let pb = mb & bb;
        if pb.not_empty() {
            return Some(pb.get_square());
        }

        //todo EP

        //knight? same trick, pretend we're a knight
        let mb = moveboard::get_move_board(moveboard::MOVEBOARD_KNIGHT, square);
        let bb = self.position.get_bit_board(PieceType::new_knight(other_color));
        let pb = mb & bb;
        if pb.not_empty() {
            return Some(pb.get_square());
        }

        //bishop?
        let p = PieceType::new_bishop(other_color);
        match self.find_specific_diagonal_attacker(square, p) {
            Some(sq) => {
                return Some(sq);
            },
            None => ()
        }
        
        //rook?
        let p = PieceType::new_rook(other_color);
        match self.find_specific_orthogonal_attacker(square, p) {
            Some(sq) => {
                return Some(sq);
            },
            None => ()
        }
        
        //queen?
        let p = PieceType::new_queen(other_color);
        match self.find_specific_orthogonal_attacker(square, p) {
            Some(sq) => {
                return Some(sq);
            },
            None => ()
        }
        match self.find_specific_diagonal_attacker(square, p) {
            Some(sq) => {
                return Some(sq);
            },
            None => ()
        }

        //king?
        let mb = moveboard::get_move_board(moveboard::MOVEBOARD_KING, square);
        let bb = self.position.get_bit_board(PieceType::new_king(other_color));
        let pb = mb & bb;
        if pb.not_empty() {
            return Some(pb.get_square());
        }

        None
    }

    pub fn is_square_attacked(&self, square: Square, color: u8) -> bool {
        let other_color = 1 - color;

        //attacked by pawn. use own color pawn capture board to intersect with opp pawns
        let mb = moveboard::get_move_board(PAWN_CAP_MOVEBOARD[color as usize], square);
        let bb = self.position.get_bit_board(PieceType::new_pawn(other_color));
        if (mb & bb).not_empty() {
            return true;
        }

        //attacked by knight
        let mb = moveboard::get_move_board(moveboard::MOVEBOARD_KNIGHT, square);
        let bb = self.position.get_bit_board(PieceType::new_knight(other_color));
        if (mb & bb).not_empty() {
            return true;
        }

        //sliding piece attacks
        if self.find_some_orthogonal_attacker(square, other_color) || self.find_some_diagonal_attacker(square, other_color) {
            return true;
        }

        //attacked by king?
        let mb = moveboard::get_move_board(moveboard::MOVEBOARD_KING, square);
        let bb = self.position.get_bit_board(PieceType::new_king(other_color));
        if (mb & bb).not_empty() {
            return true;
        }

        false
    }

    #[allow(dead_code)]
    pub fn capture_exchange(&self, square: Square) -> Position {
        let mut pos = self.position.clone();
        loop {
            let gen = Generator::new(&pos);
            match gen.find_lowest_attacker(square, 1 - pos.get_active_color()) {
                Some(attacking_sq) => {
                    let mut move_ = Move_::from_squares(attacking_sq, square);
                    //TODO promotion
                    move_ = pos.analyze_move(move_);
                    pos.apply_move(move_);
                },
                None => break
            }
        }
        pos
    }

    fn find_some_orthogonal_attacker(&self, square: Square, other_color: u8) -> bool {
        let inter = self.get_orthoganal_ray_intersect(square);

        let bb = 
            self.position.get_bit_board(PieceType::new_queen(other_color)) | 
            self.position.get_bit_board(PieceType::new_rook(other_color));
            
        return (inter & bb).not_empty();
    }

    #[allow(dead_code)]
    fn find_specific_orthogonal_attacker(&self, square: Square, piece: PieceType) -> Option<Square> {
        let inter = self.get_orthoganal_ray_intersect(square);
        let pb = self.position.get_bit_board(piece) & inter;
        if pb.not_empty() {
            return Some(pb.get_square());
        }
        None
    }

    fn get_orthoganal_ray_intersect(&self, square: Square) -> BitBoard {
        let ray_board = moveboard::get_ray_board(moveboard::DIR_UP, square);
        let mut inter = (ray_board & self.all_piece_board).keep_lowest();
        
        let ray_board = moveboard::get_ray_board(moveboard::DIR_RIGHT, square);
        inter |= (ray_board & self.all_piece_board).keep_lowest();

        let ray_board = moveboard::get_ray_board(moveboard::DIR_DOWN, square);
        inter |= (ray_board & self.all_piece_board).keep_highest();

        let ray_board = moveboard::get_ray_board(moveboard::DIR_LEFT, square);
        inter |= (ray_board & self.all_piece_board).keep_highest();

        inter
    }

    fn find_some_diagonal_attacker(&self, square: Square, other_color: u8) -> bool {
        let inter = self.get_diagonal_ray_intersect(square);

        let bb = 
            self.position.get_bit_board(PieceType::new_queen(other_color)) | 
            self.position.get_bit_board(PieceType::new_bishop(other_color));
        
        return (inter & bb).not_empty();
    }

    #[allow(dead_code)]
    fn find_specific_diagonal_attacker(&self, square: Square, piece: PieceType) -> Option<Square> {
        let inter = self.get_diagonal_ray_intersect(square);
        let pb = self.position.get_bit_board(piece) & inter;
        if pb.not_empty() {
            return Some(pb.get_square());
        }
        None
    }

    fn get_diagonal_ray_intersect(&self, square: Square) -> BitBoard {
        let ray_board = moveboard::get_ray_board(moveboard::DIR_UP_RIGHT, square);
        let mut inter = (ray_board & self.all_piece_board).keep_lowest();
        
        let ray_board = moveboard::get_ray_board(moveboard::DIR_DOWN_RIGHT, square);
        inter |= (ray_board & self.all_piece_board).keep_highest();

        let ray_board = moveboard::get_ray_board(moveboard::DIR_DOWN_LEFT, square);
        inter |= (ray_board & self.all_piece_board).keep_highest();

        let ray_board = moveboard::get_ray_board(moveboard::DIR_UP_LEFT, square);
        inter |= (ray_board & self.all_piece_board).keep_lowest();

        inter
    }
}