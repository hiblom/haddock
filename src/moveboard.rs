use crate::bitboard::BitBoard;
use crate::piecetype::PieceType;
use crate::square::Square;

pub const DIR_UP: usize = 0;
pub const DIR_UP_RIGHT: usize = 1;
pub const DIR_RIGHT: usize = 2;
pub const DIR_DOWN_RIGHT: usize = 3;
pub const DIR_DOWN: usize = 4;
pub const DIR_DOWN_LEFT: usize = 5;
pub const DIR_LEFT: usize = 6;
pub const DIR_UP_LEFT: usize = 7;


lazy_static! {
    static ref PIECE_MOVE_BOARDS: [[BitBoard; 64]; 4] = {
        println!("Haddock is generating piece move boards");

        let mut result = [[BitBoard::new(); 64]; 4];
        result[0] = create_king_moveboards();
        result[1] = create_king_moveboards();
        result[2] = create_knight_moveboards();
        result[3] = create_knight_moveboards();
        result
    };

    static ref RAY_MOVE_BOARDS: [[BitBoard; 64]; 8] = {
        println!("Haddock is generating ray move boards");

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

//translate piece values to MoveBoard index, 255 signifies no moveboard for piece
const PIECE_BOARD_INDEX: [usize; 12] = [255,255,0,1,255,255,255,255,255,255,2,3];

pub fn get_move_board(piece: PieceType, square: Square) -> BitBoard {
    PIECE_MOVE_BOARDS[PIECE_BOARD_INDEX[piece.to_usize()]][square.to_usize()]
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