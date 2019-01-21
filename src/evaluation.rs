use crate::global;
use crate::global::COLOR_WHITE;
use crate::global::COLOR_BLACK;
use crate::position::Position;
use crate::outcome::Outcome;
use crate::piecetype::PieceType;

const PIECE_VALUE: [i32; 12] = [
    100,-100, //pawn
      0,   0, //king
    900,-900, //queen
    500,-500, //rook
    300,-300, //bishop
    300,-300  //knight
];

const START_PIECE_VALUE_TOTAL:i32 = 
    16 * PIECE_VALUE[0] + //pawn
     4 * PIECE_VALUE[6] + //rook
     4 * PIECE_VALUE[8] + //bishop
     4 * PIECE_VALUE[10]; //knight

const PIECE_SQUARE_VALUE: [[[i32; 64]; 2]; 12] = [
    //white pawn    
    [
        [
          0,  0,  0,  0,  0,  0,  0,  0,
          5, 10, 10,-25,-25, 10, 10,  5,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0, 25, 25,  0,  0,  0,
          0,  0, 25, 25, 25, 25,  0,  0,
         10, 25, 25, 25, 25, 25, 25, 10,
         50, 50, 50, 50, 50, 50, 50, 50,
          0,  0,  0,  0,  0,  0,  0,  0
        ],
        [
          0,  0,  0,  0,  0,  0,  0,  0,
          5,  5,  5,-25,-25,  5,  5,  5,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0, 25, 25,  0,  0,  0,
          0,  0, 25, 25, 25, 25,  0,  0,
         10, 25, 25, 25, 25, 25, 25, 10,
         50, 50, 50, 50, 50, 50, 50, 50,
          0,  0,  0,  0,  0,  0,  0,  0
        ]
    ],
    //black pawn
    [
        [
          0,  0,  0,  0,  0,  0,  0,  0,
        -50,-50,-50,-50,-50,-50,-50,-50,
        -10,-25,-25,-25,-25,-25,-25,-10,
          0,  0,-25,-25,-25,-25,  0,  0,
          0,  0,  0,-25,-25,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
         -5,-10,-10, 25, 25,-10,-10, -5,
          0,  0,  0,  0,  0,  0,  0,  0
        ],
        [
          0,  0,  0,  0,  0,  0,  0,  0,
        -50,-50,-50,-50,-50,-50,-50,-50,
        -10,-25,-25,-25,-25,-25,-25,-10,
          0,  0,-25,-25,-25,-25,  0,  0,
          0,  0,  0,-25,-25,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
         -5, -5, -5, 25, 25, -5, -5, -5,
          0,  0,  0,  0,  0,  0,  0,  0
        ]
    ],
    //white king
    [
        [
         10, 25,  0,  0,  0,  0, 25, 10,
        -10,-10,-25,-25,-25,-25,-10,-10,
        -50,-50,-50,-50,-50,-50,-50,-50, 
        -50,-50,-50,-50,-50,-50,-50,-50,
        -50,-50,-50,-50,-50,-50,-50,-50,       
        -50,-50,-50,-50,-50,-50,-50,-50,
        -50,-50,-50,-50,-50,-50,-50,-50,
        -50,-50,-50,-50,-50,-50,-50,-50
        ],
        [
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
        ]
    ],
    //black king
    [
        [
         50, 50, 50, 50, 50, 50, 50, 50, 
         50, 50, 50, 50, 50, 50, 50, 50,
         50, 50, 50, 50, 50, 50, 50, 50,       
         50, 50, 50, 50, 50, 50, 50, 50,
         50, 50, 50, 50, 50, 50, 50, 50,
         50, 50, 50, 50, 50, 50, 50, 50,
         10, 10, 25, 25, 25, 25, 10, 10,
        -10,-25,  0,  0,  0,  0,-25,-10
        ],
        [
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
        ]
    ],
    //white queen
    [
        [
        -50,-25,-25,  0,  0,-25,-25,-50,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,       
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -50,-25,-25,  0,  0,-25,-25,-50
        ],
        [
        -50,-25,-25,  0,  0,-25,-25,-50,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0, 10, 25, 25, 10,  0,-25,       
        -25,  0, 25, 25, 25, 25,  0,-25,
        -25,  0, 25, 25, 25, 25,  0,-25,
        -25,  0, 25, 25, 25, 25,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -50,-25,-25,  0,  0,-25,-25,-50
        ]
    ],
    //black queen
    [
        [
        50, 25, 25,  0,  0, 25, 25, 50,
        25,  0,  0,  0,  0,  0,  0, 25,
        25,  0,  0,  0,  0,  0,  0, 25,       
        25,  0,  0,  0,  0,  0,  0, 25,
        25,  0,  0,  0,  0,  0,  0, 25,
        25,  0,  0,  0,  0,  0,  0, 25,
        25,  0,  0,  0,  0,  0,  0, 25,
        50, 25, 25,  0,  0, 25, 25, 50
        ],
        [
        50, 25, 25,  0,  0, 25, 25, 50,
        25,  0,  0,  0,  0,  0,  0, 25,
        25,  0,-25,-25,-25,-25,  0, 25,       
        25,  0,-25,-25,-25,-25,  0, 25,
        25,  0,-25,-25,-25,-25,  0, 25,
        25,  0,-10,-25,-25,-10,  0, 25,
        25,  0,  0,  0,  0,  0,  0, 25,
        50, 25, 25,  0,  0, 25, 25, 50
        ]
    ],
    //white rook
    [
        [
        -10,  0,  0, 10, 10, 10,  0,-10,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,       
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,
          0,  0,  0,  0,  0,  0,  0,  0,
        ],
        [
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
        ]
    ],
    //black rook
    [
        [
          0,  0,  0,  0,  0,  0,  0,  0,
         25,  0,  0,  0,  0,  0,  0, 25,
         25,  0,  0,  0,  0,  0,  0, 25,       
         25,  0,  0,  0,  0,  0,  0, 25,
         25,  0,  0,  0,  0,  0,  0, 25,
         25,  0,  0,  0,  0,  0,  0, 25,
         25,  0,  0,  0,  0,  0,  0, 25,
         10,  0,  0,-10,-10,-10,  0, 10,
        ],
        [
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
          0,  0,  0,  0,  0,  0,  0,  0,
        ]
    ],
    //white bishop
    [
        [
        -50,-25,-25,  0,  0,-25,-25,-50,
        -25, 10,  0,  0,  0,  0, 10,-25,
        -25,  0, 25,  0,  0, 25,  0,-25,       
        -25,  0,  0, 25, 25,  0,  0,-25,
        -25,  0,  0, 25, 25,  0,  0,-25,
        -25,  0, 25,  0,  0, 25,  0,-25,
        -25, 10,  0,  0,  0,  0, 10,-25,
        -50,-25,-25,  0,  0,-25,-25,-50
        ],
        [
        -50,-25,-25,  0,  0,-25,-25,-50,
        -25, 10,  0,  0,  0,  0, 10,-25,
        -25,  0, 25,  0,  0, 25,  0,-25,       
        -25,  0,  0, 25, 25,  0,  0,-25,
        -25,  0,  0, 25, 25,  0,  0,-25,
        -25,  0, 25,  0,  0, 25,  0,-25,
        -25, 10,  0,  0,  0,  0, 10,-25,
        -50,-25,-25,  0,  0,-25,-25,-50
        ]
    ],
    //black bishop
    [
        [
        50, 25, 25,  0,  0, 25, 25, 50,
        25,-10,  0,  0,  0,  0,-10, 25,
        25,  0,-25,  0,  0,-25,  0, 25,       
        25,  0,  0,-25,-25,  0,  0, 25,
        25,  0,  0,-25,-25,  0,  0, 25,
        25,  0,-25,  0,  0,-25,  0, 25,
        25,-10,  0,  0,  0,  0,-10, 25,
        50, 25, 25,  0,  0, 25, 25, 50
        ],
        [
        50, 25, 25,  0,  0, 25, 25, 50,
        25,-10,  0,  0,  0,  0,-10, 25,
        25,  0,-25,  0,  0,-25,  0, 25,
        25,  0,  0,-25,-25,  0,  0, 25,
        25,  0,  0,-25,-25,  0,  0, 25,
        25,  0,-25,  0,  0,-25,  0, 25,
        25,-10,  0,  0,  0,  0,-10, 25,
        50, 25, 25,  0,  0, 25, 25, 50
        ]
    ],
    //white knight
    [
        [
        -50,-25,-25,-25,-25,-25,-25,-50,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0, 25, 25, 25, 25,  0,-25,       
        -25,  0, 26, 25, 25, 25,  0,-25,
        -25,  0, 25, 25, 25, 25,  0,-25,
        -25,  0, 25, 25, 25, 25,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -50,-25,-25,-25,-25,-25,-25,-50
        ],
        [
        -50,-25,-25,-25,-25,-25,-25,-50,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -25,  0, 25, 25, 25, 25,  0,-25,       
        -25,  0, 26, 25, 25, 25,  0,-25,
        -25,  0, 25, 25, 25, 25,  0,-25,
        -25,  0, 25, 25, 25, 25,  0,-25,
        -25,  0,  0,  0,  0,  0,  0,-25,
        -50,-25,-25,-25,-25,-25,-25,-50
        ]
    ],
    //black knight
    [
        [
         50, 25, 25, 25, 25, 25, 25, 50,
         25,  0,  0,  0,  0,  0,  0, 25,
         25,  0,-25,-25,-25,-25,  0, 25,
         25,  0,-25,-25,-25,-25,  0, 25,
         25,  0,-25,-25,-25,-25,  0, 25,       
         25,  0,-25,-25,-25,-25,  0, 25,
         25,  0,  0,  0,  0,  0,  0, 25,
         50, 25, 25, 25, 25, 25, 25, 50
        ],
        [
         50, 25, 25, 25, 25, 25, 25, 50,
         25,  0,  0,  0,  0,  0,  0, 25,
         25,  0,-25,-25,-25,-25,  0, 25,
         25,  0,-25,-25,-25,-25,  0, 25,
         25,  0,-25,-25,-25,-25,  0, 25,       
         25,  0,-25,-25,-25,-25,  0, 25,
         25,  0,  0,  0,  0,  0,  0, 25,
         50, 25, 25, 25, 25, 25, 25, 50
        ]
    ]
];

pub fn evaluate(position: &Position, depth: i32) -> Outcome {
    //check status of other king. when check, then the outcome is illegal
    //let color = position.get_active_color();

    /*
    if do_legal_check && Generator::new(position).is_check(other_color) {
        return Outcome::Illegal(depth)
    }
    */

    /*
    let check = is_check(position, color);
    
    let no_legal_moves_left = generator::generate_legal_moves(position).len() == 0;

    let check_mate = check && no_legal_moves_left;
    if check_mate {
        if color == global::COLOR_WHITE {
            return Outcome::WhiteIsMate(0)
        }
        else {
            return Outcome::BlackIsMate(0)
        }
    }

    let stale_mate = !check && no_legal_moves_left;
    if stale_mate {
        return Outcome::DrawByStalemate
    }
    */

    let halfmoveclock = position.get_halfmoveclock() >= global::MAX_HALFMOVECLOCK;
    if halfmoveclock {
        return Outcome::DrawByHalfmoveclock(depth)
    }

    if position.is_three_fold_repetition() {
        return Outcome::DrawByRepetition(depth);
    }
    
    //TODO not enough material

    let mut material_value = get_material_value(position);

    //penalty when two fold repetition, and we are ahead, we do not want to give opponent chance to have three fold
    if position.is_two_fold_repetition() {
        if position.get_active_color() == 0 && material_value > 0 || position.get_active_color() == 1 && material_value < 0 {
            material_value = 0;
        }
    }

    Outcome::Undecided(depth, material_value)
}

fn get_material_value(position: &Position) -> i32 {
    let mut value: i32 = 0;
    let pos_value_total = get_pos_value_total(position);

    for (piece, square) in position.get_all_active_pieces() {
        value += PIECE_VALUE[piece.to_usize()] + 
        (
            PIECE_SQUARE_VALUE[piece.to_usize()][0][square.to_usize()] * pos_value_total +
            PIECE_SQUARE_VALUE[piece.to_usize()][1][square.to_usize()] * (START_PIECE_VALUE_TOTAL - pos_value_total)
        ) / START_PIECE_VALUE_TOTAL;
    }

    //println!("pos total {} material value is {}", pos_value_total, value);

    value
}

fn get_pos_value_total(position: &Position) -> i32 {
    return
        (
            position.get_piece_count(PieceType::new_pawn(COLOR_WHITE)) + 
            position.get_piece_count(PieceType::new_pawn(COLOR_BLACK))
        ) * PIECE_VALUE[PieceType::new_pawn(COLOR_WHITE).to_usize()] +
        (
            position.get_piece_count(PieceType::new_rook(COLOR_WHITE)) + 
            position.get_piece_count(PieceType::new_rook(COLOR_BLACK))
        ) * PIECE_VALUE[PieceType::new_rook(COLOR_WHITE).to_usize()] +
        (
            position.get_piece_count(PieceType::new_bishop(COLOR_WHITE)) + 
            position.get_piece_count(PieceType::new_bishop(COLOR_BLACK))
        ) * PIECE_VALUE[PieceType::new_bishop(COLOR_WHITE).to_usize()] +
        (
            position.get_piece_count(PieceType::new_knight(COLOR_WHITE)) + 
            position.get_piece_count(PieceType::new_knight(COLOR_BLACK))
        ) * PIECE_VALUE[PieceType::new_knight(COLOR_WHITE).to_usize()];
}