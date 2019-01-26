extern crate rand;
extern crate lazy_static;

const SEED_SQUARE_PIECE_KEYS: &str = "k>FHVV_Bpjqw)TMTs/'4f'FKJ7\\qN{va";
const SEED_BLACK_KEY: &str = "g@/,hx'#]]^k#6qCM`Bd#3-Fv#Lu_Qj;";
const SEED_CASTLE_KEYS: &str = ")#B&3qTwtG{Wgy6Vn:xCxSy+CY88\\Q\"^";
const SEED_EP_FILE_KEYS: &str = "XCkLM'a])qN3wMDs+2d6x2RTHt7NZX#s";

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

lazy_static! {
    pub static ref ZOBRIST_SQUARE_PIECE_KEYS: [[u64; 12]; 64] = {
        println!("initalizing ZOBRIST_SQUARE_PIECE_KEYS");
        let mut result: [[u64; 12]; 64] = [[0; 12]; 64] ;
        let mut rng: StdRng = SeedableRng::from_seed(seed_str_to_arr(SEED_SQUARE_PIECE_KEYS));
        for i in 0..64 {
            for j in 0..12 {
                result[i][j] = rng.gen::<u64>();
            }
        }
        result
    };

    pub static ref ZOBRIST_BLACK_KEY: [u64; 1] = {
        println!("initalizing ZOBRIST_BLACK_KEY");
        let mut rng: StdRng = SeedableRng::from_seed(seed_str_to_arr(SEED_BLACK_KEY));
        [rng.gen::<u64>()]
    };

    pub static ref ZOBRIST_CASTLE_KEYS: [u64; 4] = {
        println!("initalizing ZOBRIST_CASTLE_KEYS");
        let mut result: [u64; 4] = [0; 4];
        let mut rng: StdRng = SeedableRng::from_seed(seed_str_to_arr(SEED_CASTLE_KEYS));
        for i in 0..4 {
            result[i] = rng.gen::<u64>();
        }
        result
    };

    pub static ref ZOBRIST_EP_FILE_KEYS: [u64; 8] = {
        println!("initalizing ZOBRIST_EP_FILE_KEYS");
        let mut result: [u64; 8] = [0; 8];
        let mut rng: StdRng = SeedableRng::from_seed(seed_str_to_arr(SEED_EP_FILE_KEYS));
        for i in 0..8 {
            result[i] = rng.gen::<u64>();
        }
        result
    };
}

fn seed_str_to_arr(seed_string: &str) -> [u8; 32] {
    let mut result: [u8; 32] = [0; 32];
    for (i, c) in seed_string.chars().enumerate() {
        result[i] = c as u8;
    }
    result
}
