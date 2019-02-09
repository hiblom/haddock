use crate::bitboard::BitBoard;
//use crate::piecetype::PieceType;
use crate::square::Square;

pub const DIR_UP: usize = 0;
pub const DIR_UP_RIGHT: usize = 1;
pub const DIR_RIGHT: usize = 2;
pub const DIR_DOWN_RIGHT: usize = 3;
pub const DIR_DOWN: usize = 4;
pub const DIR_DOWN_LEFT: usize = 5;
pub const DIR_LEFT: usize = 6;
pub const DIR_UP_LEFT: usize = 7;

pub const MOVEBOARD_KING: usize = 0;
pub const MOVEBOARD_KNIGHT: usize = 1;
pub const MOVEBOARD_WHITE_PAWN_PUSH: usize = 2;
pub const MOVEBOARD_BLACK_PAWN_PUSH: usize = 3;
pub const MOVEBOARD_WHITE_PAWN_CAP: usize = 4;
pub const MOVEBOARD_BLACK_PAWN_CAP: usize = 5;


lazy_static! {
    static ref PIECE_MOVE_BOARDS: [[BitBoard; 64]; 6] = {
        let mut result = [[BitBoard::new(); 64]; 6];
        result[MOVEBOARD_KING] = create_king_moveboards();
        result[MOVEBOARD_KNIGHT] = create_knight_moveboards();
        result[MOVEBOARD_WHITE_PAWN_PUSH] = create_white_pawn_moveboards();
        result[MOVEBOARD_BLACK_PAWN_PUSH] = create_black_pawn_moveboards();
        result[MOVEBOARD_WHITE_PAWN_CAP] = create_white_pawn_captureboards();
        result[MOVEBOARD_BLACK_PAWN_CAP] = create_black_pawn_captureboards();
        result
    };

    static ref RAY_MOVE_BOARDS: [[BitBoard; 64]; 8] = {
        let mut result = [[BitBoard::new(); 64]; 8];
        result[DIR_UP] = create_ray_up_moveboards();
        result[DIR_UP_RIGHT] = create_ray_up_right_moveboards();
        result[DIR_RIGHT] = create_ray_right_moveboards();
        result[DIR_DOWN_RIGHT] = create_ray_down_right_moveboards();
        result[DIR_DOWN] = create_ray_down_moveboards();
        result[DIR_DOWN_LEFT] = create_ray_down_left_moveboards();
        result[DIR_LEFT] = create_ray_left_moveboards();
        result[DIR_UP_LEFT] = create_ray_up_left_moveboards();
        result
    };

}

pub fn get_move_board(mb: usize, square: Square) -> BitBoard {
    PIECE_MOVE_BOARDS[mb][square.to_usize()]
}

pub fn get_ray_board(dir: usize, square: Square) -> BitBoard {
    RAY_MOVE_BOARDS[dir][square.to_usize()]
}

fn create_king_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);
        bb |= bb.clone().left() | bb.clone().right();
        bb |= bb.clone().up() | bb.clone().down();
        bb.clear(sq);
        result[i as usize] = bb;
    }
    result
}

fn create_knight_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut b_vert = result[i as usize];
        b_vert.set(sq);
        b_vert = b_vert.clone().left() | b_vert.clone().right();
        b_vert = b_vert.clone().up().up() | b_vert.clone().down().down();

        let mut b_hor = result[i as usize];
        b_hor.set(sq);
        b_hor = b_hor.clone().up() | b_hor.clone().down();
        b_hor = b_hor.clone().left().left() | b_hor.clone().right().right();

        result[i as usize] = b_vert | b_hor;
    }
    result
}

fn create_white_pawn_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);
        bb.up();

        result[i as usize] = bb;
    }
    result
}

fn create_black_pawn_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);
        bb.down();

        result[i as usize] = bb;
    }
    result
}

fn create_white_pawn_captureboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);
        let bl = bb.clone().up().left();
        let br = bb.clone().up().right();

        result[i as usize] = bl | br;
    }
    result
}

fn create_black_pawn_captureboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);
        let bl = bb.clone().down().left();
        let br = bb.clone().down().right();

        result[i as usize] = bl | br;
    }
    result
}

fn create_ray_up_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);

        let mut bs = bb.clone().up();
        while bs.not_empty() {
            bb |= bs;
            bs.up();
        }
        bb.clear(sq);

        result[i as usize] = bb;
    }
    result
}

fn create_ray_down_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);

        let mut bs = bb.clone().down();
        while bs.not_empty() {
            bb |= bs;
            bs.down();
        }
        bb.clear(sq);

        result[i as usize] = bb;
    }
    result
}

fn create_ray_left_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);

        let mut bs = bb.clone().left();
        while bs.not_empty() {
            bb |= bs;
            bs.left();
        }
        bb.clear(sq);

        result[i as usize] = bb;
    }
    result
}

fn create_ray_right_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);

        let mut bs = bb.clone().right();
        while bs.not_empty() {
            bb |= bs;
            bs.right();
        }
        bb.clear(sq);

        result[i as usize] = bb;
    }
    result
}

fn create_ray_up_right_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);

        let mut bs = bb.clone().up().right();
        while bs.not_empty() {
            bb |= bs;
            bs.up();
            bs.right();
        }
        bb.clear(sq);

        result[i as usize] = bb;
    }
    result
}

fn create_ray_down_right_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);

        let mut bs = bb.clone().down().right();
        while bs.not_empty() {
            bb |= bs;
            bs.down();
            bs.right();
        }
        bb.clear(sq);

        result[i as usize] = bb;
    }
    result
}

fn create_ray_down_left_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);

        let mut bs = bb.clone().down().left();
        while bs.not_empty() {
            bb |= bs;
            bs.down();
            bs.left();
        }
        bb.clear(sq);

        result[i as usize] = bb;
    }
    result
}

fn create_ray_up_left_moveboards() -> [BitBoard; 64] {
    let mut result = [BitBoard::new(); 64];
    for i in 0u8..64 {
        let sq = Square::new(i);
        let mut bb = result[i as usize];
        bb.set(sq);

        let mut bs = bb.clone().up().left();
        while bs.not_empty() {
            bb |= bs;
            bs.up();
            bs.left();
        }
        bb.clear(sq);

        result[i as usize] = bb;
    }
    result
}