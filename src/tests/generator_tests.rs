#[allow(unused_imports)]
use crate::generator::Generator;
#[allow(unused_imports)]
use crate::global::COLOR_WHITE;
#[allow(unused_imports)]
use crate::piecetype::PieceType;

fn mv(pos: &crate::position::Position, mv_str: &str) -> crate::move_::Move_ {
    let move_ = crate::move_::Move_::from_str(mv_str).unwrap();
    pos.analyze_move(move_)
}

fn sq(sq_str: &str) -> crate::square::Square {
    crate::square::Square::from_str(sq_str).unwrap()
}

#[test]
fn test_king_moves_middle() {
    //arrange

    //create board white king at d4
    let fen = "8/8/8/8/3K4/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 8] = [
        mv(&position, "d4c5"),
        mv(&position, "d4d5"),
        mv(&position, "d4e5"),
        mv(&position, "d4c4"),
        mv(&position, "d4e4"),
        mv(&position, "d4c3"),
        mv(&position, "d4d3"),
        mv(&position, "d4e3"),
    ];

    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("d4"), PieceType::new_king(COLOR_WHITE), false, &mut moves);

    //assert
    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of king moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "King moves not as expected");
    }
}


#[test]
fn test_king_moves_corner() {
    //arrange

    //create board with only white king
    let fen = "8/8/8/8/8/8/8/K7 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 3] = [
        mv(&position, "a1a2"),
        mv(&position, "a1b1"),
        mv(&position, "a1b2"),
    ];

    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("a1"), PieceType::new_king(COLOR_WHITE), false, &mut moves);

    //assert
    println!("expected_moves moves:");
    for move_ in expected_moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }

    println!("generated moves:");
    for move_ in moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }


    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of king moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "King moves not as expected");
    }
}

#[test]
fn test_king_moves_other_pieces() {
    //arrange

    //create board with white king, white pawn, black pawn
    let fen = "8/8/8/8/8/8/Pp6/K7 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 2] = [
        mv(&position, "a1b1"),
        mv(&position, "a1b2"),
    ];

    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("a1"), PieceType::new_king(COLOR_WHITE), false, &mut moves);

    //assert
    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of king moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "King moves not as expected");
    }
}

#[test]
fn test_rook_moves_middle() {
    //arrange

    //create board with white rook on d4
    let fen = "8/8/8/8/3R4/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 14] = [
        mv(&position, "d4d1"),
        mv(&position, "d4d2"),
        mv(&position, "d4d3"),
        mv(&position, "d4d5"),
        mv(&position, "d4d6"),
        mv(&position, "d4d7"),
        mv(&position, "d4d8"),
        mv(&position, "d4a4"),
        mv(&position, "d4b4"),
        mv(&position, "d4c4"),
        mv(&position, "d4e4"),
        mv(&position, "d4f4"),
        mv(&position, "d4g4"),
        mv(&position, "d4h4"),
    ];

    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("d4"), PieceType::new_rook(COLOR_WHITE), false, &mut moves);

    //assert
    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of rook moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Rook moves not as expected");
    }
}

#[test]
fn test_rook_moves_corner() {
    //arrange

    //create board with white rook on h8
    let fen = "7R/8/8/8/8/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 14] = [
        mv(&position, "h8h1"),
        mv(&position, "h8h2"),
        mv(&position, "h8h3"),
        mv(&position, "h8h4"),
        mv(&position, "h8h5"),
        mv(&position, "h8h6"),
        mv(&position, "h8h7"),
        mv(&position, "h8a8"),
        mv(&position, "h8b8"),
        mv(&position, "h8c8"),
        mv(&position, "h8d8"),
        mv(&position, "h8e8"),
        mv(&position, "h8f8"),
        mv(&position, "h8g8"),
    ];

    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("h8"), PieceType::new_rook(COLOR_WHITE), false, &mut moves);

    //assert
    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of rook moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Rook moves not as expected");
    }
}

#[test]
fn test_rook_moves_other_pieces() {
    //arrange

    //create board with white rook h8, black pawn on h7, white bishop on e8
    let fen = "4B2R/7p/8/8/8/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 3] = [
        mv(&position, "h8f8"),
        mv(&position, "h8g8"),
        mv(&position, "h8h7"), //capture
    ];

    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("h8"), PieceType::new_rook(COLOR_WHITE), false, &mut moves);

    //assert
    println!("expected_moves moves:");
    for move_ in expected_moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }

    println!("generated moves:");
    for move_ in moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }

    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of rook moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Rook moves not as expected");
    }
}

#[test]
fn test_knight_moves_middle() {
    //arrange

    //create board with white knight on d4
    let fen = "8/8/8/8/3N4/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 8] = [
        mv(&position, "d4c2"),
        mv(&position, "d4e2"),
        mv(&position, "d4b3"),
        mv(&position, "d4f3"),
        mv(&position, "d4b5"),
        mv(&position, "d4f5"),
        mv(&position, "d4c6"),
        mv(&position, "d4e6"),
    ];

    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("d4"), PieceType::new_knight(COLOR_WHITE), false, &mut moves);

    //assert
    println!("expected_moves moves:");
    for move_ in expected_moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }

    println!("generated moves:");
    for move_ in moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }

    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of knight moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(
            moves.contains(expected_move),
            "Knight moves not as expected"
        );
    }
}


#[test]
fn test_knight_moves_corner() {
    //arrange

    //create board with white knight on h1
    let fen = "8/8/8/8/8/8/8/7N w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 2] = [
        mv(&position, "h1f2"),
        mv(&position, "h1g3"),
    ];

    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("h1"), PieceType::new_knight(COLOR_WHITE), false, &mut moves);

    //assert
    println!("expected_moves moves:");
    for move_ in expected_moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }

    println!("generated moves:");
    for move_ in moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }

    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of knight moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Knight moves not as expected");
    }
}

#[test]
fn test_knight_moves_startpos_b1() {
    //arrange

    //create start pos board, calc knight moves of b1
    let position = crate::parser::parse_startpos().unwrap();

    let expected_moves: [crate::move_::Move_; 2] = [
        mv(&position, "b1a3"),
        mv(&position, "b1c3"),
    ];

    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("b1"), PieceType::new_knight(COLOR_WHITE), false, &mut moves);

    //assert
    println!("expected_moves moves:");
    for move_ in expected_moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }

    println!("generated moves:");
    for move_ in moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }

    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of knight moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Knight moves not as expected");
    }
}


#[test]
fn test_pawn_moves_startpos_e2() {
    //arrange
    let position = crate::parser::parse_startpos().unwrap();

    let expected_moves: [crate::move_::Move_; 2] = [
        mv(&position, "e2e3"),
        mv(&position, "e2e4"),
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_pawn_moves_2(false, &mut moves);

    //assert
    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Pawn moves not as expected");
    }
}

#[test]
fn test_pawn_moves_startpos_e7() {
    //arrange
    let mut position = crate::parser::parse_startpos().unwrap();
    position.set_active_color(crate::global::COLOR_BLACK);

    let expected_moves: [crate::move_::Move_; 2] = [
        mv(&position, "e7e6"),
        mv(&position, "e7e5"),
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_pawn_moves_2(false, &mut moves);

    //assert
    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Pawn moves not as expected");
    }
}

#[test]
fn test_pawn_moves_capture() {
    //arrange
    //white pawn d4, black pawn e5
    let fen = "8/8/8/4p3/3P4/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 2] = [
        mv(&position, "d4d5"),
        mv(&position, "d4e5"),
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_pawn_moves_2(false, &mut moves);

    //assert
    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Pawn moves not as expected");
    }
}

#[test]
fn test_bishop_moves_middle() {
    //arrange
    //white bishop d4
    let fen = "8/8/8/8/3B4/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 13] = [
        mv(&position, "d4a1"),
        mv(&position, "d4b2"),
        mv(&position, "d4c3"),
        mv(&position, "d4e5"),
        mv(&position, "d4f6"),
        mv(&position, "d4g7"),
        mv(&position, "d4h8"),
        mv(&position, "d4a7"),
        mv(&position, "d4b6"),
        mv(&position, "d4c5"),
        mv(&position, "d4e3"),
        mv(&position, "d4f2"),
        mv(&position, "d4g1"),
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("d4"), PieceType::new_bishop(COLOR_WHITE), false, &mut moves);

    //assert
    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of pawn moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Pawn moves not as expected");
    }
}

#[test]
fn test_bishop_moves_edge() {
    //arrange
    //white bishop h4
    let fen = "8/8/8/8/7B/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 7] = [
        mv(&position, "h4d8"),
        mv(&position, "h4e7"),
        mv(&position, "h4f6"),
        mv(&position, "h4g5"),
        mv(&position, "h4g3"),
        mv(&position, "h4f2"),
        mv(&position, "h4e1"),
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("h4"), PieceType::new_bishop(COLOR_WHITE), false, &mut moves);

    //assert
    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of pawn moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Bishop moves not as expected");
    }
}

#[test]
fn test_bishop_moves_other_pieces() {
    //arrange
    //white bishop h4, white pawn f6, black bishop f2
    let fen = "8/8/5P2/8/7B/8/5b2/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 3] = [
        mv(&position, "h4g5"),
        mv(&position, "h4g3"),
        mv(&position, "h4f2"),
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_piece_moves(sq("h4"), PieceType::new_bishop(COLOR_WHITE), false, &mut moves);

    //assert
    println!("expected_moves moves:");
    for move_ in expected_moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }

    println!("generated moves:");
    for move_ in moves.iter() {
        println!("{}", crate::move_::Move_::to_fen(*move_));
    }


    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of pawn moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Pawn moves not as expected");
    }
}


#[test]
fn test_pawn_moves_ep() {
    //arrange
    //white pawn on d5, black pawn e5, en-passant square is e6
    let fen = "8/8/8/3Pp3/8/8/8/8 w - e6 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 2] = [
        mv(&position, "d5d6"),
        mv(&position, "d5e6"),
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_pawn_moves_2(false, &mut moves);

    //assert
    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Pawn moves not as expected");
    }
}

#[test]
fn test_pawn_moves_promo() {
    //arrange
    let fen = "k7/4P3/8/8/8/8/8/K7 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 4] = [
        mv(&position, "e7e8q"),
        mv(&position, "e7e8r"),
        mv(&position, "e7e8b"),
        mv(&position, "e7e8n")
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_pawn_moves_2(false, &mut moves);

    //assert
    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "Pawn moves not as expected");
    }
}

#[test]
fn test_king_castling() {
    //arrange
    //black to move, king can go to d8 or c8 (castle)
    let fen = "r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 2] = [
        position.analyze_move(mv(&position, "e8d8")),
        position.analyze_move(mv(&position, "e8c8"))
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_king_moves(sq("e8"), false, &mut moves);

    //assert
    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of king moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "king moves not as expected");
    }

}

#[test]
fn test_king_castling_sq_taken() {
    //arrange
    //black to move, king cannot castle because bishop is on b8
    let fen = "rb2kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 1] = [
        mv(&position, "e8d8")
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_king_moves(sq("e8"), false, &mut moves);

    //assert
    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of king moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "king moves not as expected");
    }

}

#[test]
fn test_king_castling_no_status() {
    //arrange
    //black to move, king cannot castle no status
    let fen = "r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQk - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [crate::move_::Move_; 1] = [
        mv(&position, "e8d8")
    ];
    
    //act
    let mut moves = Vec::new();
    Generator::new(&position).generate_king_moves(sq("e8"), false, &mut moves);

    //assert
    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of king moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "king moves not as expected");
    }

}

/*

#[test]
fn test_legal_moves() {
    //arrange
    //black to move, rook on g1, black king h8
    let fen = "7k/8/8/8/8/8/8/K5R1 b - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    //only one legal move
    let expected_moves: [crate::move_::Move_; 1] = [
        mv(&position, "h8h7")
    ];
    
    //act
    let moves = Generator::new(&position).generate_legal_moves(&position);

    //assert
    assert_eq!(
        expected_moves.len(),
        moves.len(),
        "Number of king moves not as expected"
    );

    for expected_move in expected_moves.iter() {
        assert!(moves.contains(expected_move), "king moves not as expected: {}", expected_move);
    }

}

*/