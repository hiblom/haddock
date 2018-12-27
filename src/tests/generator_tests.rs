fn mv(mv_str: &str) -> u32 {
    crate::move_::Move_::from_str(mv_str).unwrap()
}

fn sq(sq_str: &str) -> u8 {
    crate::square::Square::from_str(sq_str).unwrap()
}

#[test]
fn test_king_moves_middle() {
    //arrange

    //create board white king at d4
    let fen = "8/8/8/8/3K4/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts);

    let expected_moves: [u32; 8] = [
        mv("d4c5"),
        mv("d4d5"),
        mv("d4e5"),
        mv("d4c4"),
        mv("d4e4"),
        mv("d4c3"),
        mv("d4d3"),
        mv("d4e3"),
    ];

    //act
    let moves = match position {
        Some(pos) => {
            crate::generator::generate_king_moves(&pos, sq("d4"), crate::global::COLOR_WHITE)
        }
        None => Vec::new(),
    };

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
    let position = crate::parser::parse_fen(&fen_parts);

    let expected_moves: [u32; 3] = [
        mv("a1a2"),
        mv("a1b1"),
        mv("a1b2"),
    ];

    //act
    let moves = match position {
        Some(pos) => crate::generator::generate_king_moves(&pos, sq("a1"), crate::global::COLOR_WHITE),
        None => Vec::new(),
    };

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
fn test_king_moves_other_pieces() {
    //arrange

    //create board with white king, white pawn, black pawn
    let fen = "8/8/8/8/8/8/Pp6/K7 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts);

    let expected_moves: [u32; 2] = [
        mv("a1b1"),
        mv("a1b2"),
    ];

    //act
    let moves = match position {
        Some(pos) => crate::generator::generate_king_moves(&pos, sq("a1"), crate::global::COLOR_WHITE),
        None => Vec::new(),
    };

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
    let position = crate::parser::parse_fen(&fen_parts);

    let expected_moves: [u32; 14] = [
        mv("d4d1"),
        mv("d4d2"),
        mv("d4d3"),
        mv("d4d5"),
        mv("d4d6"),
        mv("d4d7"),
        mv("d4d8"),
        mv("d4a4"),
        mv("d4b4"),
        mv("d4c4"),
        mv("d4e4"),
        mv("d4f4"),
        mv("d4g4"),
        mv("d4h4"),
    ];

    //act
    let moves = match position {
        Some(pos) => crate::generator::generate_rook_moves(&pos, sq("d4"), crate::global::COLOR_WHITE),
        None => Vec::new(),
    };

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
    let position = crate::parser::parse_fen(&fen_parts);

    let expected_moves: [u32; 14] = [
        mv("h8h1"),
        mv("h8h2"),
        mv("h8h3"),
        mv("h8h4"),
        mv("h8h5"),
        mv("h8h6"),
        mv("h8h7"),
        mv("h8a8"),
        mv("h8b8"),
        mv("h8c8"),
        mv("h8d8"),
        mv("h8e8"),
        mv("h8f8"),
        mv("h8g8"),
    ];

    //act
    let moves = match position {
        Some(pos) => crate::generator::generate_rook_moves(&pos, sq("h8"), crate::global::COLOR_WHITE),
        None => Vec::new(),
    };

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
    let position = crate::parser::parse_fen(&fen_parts);

    let expected_moves: [u32; 3] = [
        mv("h8f8"),
        mv("h8g8"),
        mv("h8h7"), //capture
    ];

    //act
    let moves = match position {
        Some(pos) => crate::generator::generate_rook_moves(&pos, sq("h8"), crate::global::COLOR_WHITE),
        None => Vec::new(),
    };

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
fn test_knight_moves_middle() {
    //arrange

    //create board with white knight on d4
    let fen = "8/8/8/8/3N4/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts);

    let expected_moves: [u32; 8] = [
        mv("d4c2"),
        mv("d4e2"),
        mv("d4b3"),
        mv("d4f3"),
        mv("d4b5"),
        mv("d4f5"),
        mv("d4c6"),
        mv("d4e6"),
    ];

    //act
    let moves = match position {
        Some(pos) => crate::generator::generate_knight_moves(&pos, sq("d4"), crate::global::COLOR_WHITE),
        None => Vec::new(),
    };

    //assert
    println!("expected_moves moves:");
    for move_ in expected_moves.iter() {
        println!("{}", crate::move_::Move_::get_fen(*move_));
    }

    println!("generated moves:");
    for move_ in moves.iter() {
        println!("{}", crate::move_::Move_::get_fen(*move_));
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
    let position = crate::parser::parse_fen(&fen_parts);

    let expected_moves: [u32; 2] = [
        mv("h1f2"),
        mv("h1g3"),
    ];

    //act
    let moves = match position {
        Some(pos) => {
            crate::generator::generate_knight_moves(&pos, sq("h1"), crate::global::COLOR_WHITE)
        }
        None => Vec::new(),
    };

    //assert
    println!("expected_moves moves:");
    for move_ in expected_moves.iter() {
        println!("{}", crate::move_::Move_::get_fen(*move_));
    }

    println!("generated moves:");
    for move_ in moves.iter() {
        println!("{}", crate::move_::Move_::get_fen(*move_));
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

    let expected_moves: [u32; 2] = [
        mv("b1a3"),
        mv("b1c3"),
    ];

    //act
    let moves = crate::generator::generate_knight_moves(&position, sq("b1"), crate::global::COLOR_WHITE);

    //assert
    println!("expected_moves moves:");
    for move_ in expected_moves.iter() {
        println!("{}", crate::move_::Move_::get_fen(*move_));
    }

    println!("generated moves:");
    for move_ in moves.iter() {
        println!("{}", crate::move_::Move_::get_fen(*move_));
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

    let expected_moves: [u32; 2] = [
        mv("e2e3"),
        mv("e2e4"),
    ];
    
    //act
    let moves = crate::generator::generate_pawn_moves(&position, sq("e2"), crate::global::COLOR_WHITE);

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
fn test_pawn_moves_startpos_e7() {
    //arrange
    let position = crate::parser::parse_startpos().unwrap();

    let expected_moves: [u32; 2] = [
        mv("e7e6"),
        mv("e7e5"),
    ];
    
    //act
    let moves = crate::generator::generate_pawn_moves(&position, sq("e7"), crate::global::COLOR_BLACK);

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
fn test_pawn_moves_capture() {
    //arrange
    //white pawn d4, black pawn e5
    let fen = "8/8/8/4p3/3P4/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [u32; 2] = [
        mv("d4d5"),
        mv("d4e5"),
    ];
    
    //act
    let moves = crate::generator::generate_pawn_moves(&position, sq("d4"), crate::global::COLOR_WHITE);

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
fn test_bishop_moves_middle() {
    //arrange
    //white bishop d4
    let fen = "8/8/8/8/3B4/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [u32; 13] = [
        mv("d4a1"),
        mv("d4b2"),
        mv("d4c3"),
        mv("d4e5"),
        mv("d4f6"),
        mv("d4g7"),
        mv("d4h8"),
        mv("d4a7"),
        mv("d4b6"),
        mv("d4c5"),
        mv("d4e3"),
        mv("d4f2"),
        mv("d4g1"),
    ];
    
    //act
    let moves = crate::generator::generate_bishop_moves(&position, sq("d4"), crate::global::COLOR_WHITE);

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

    let expected_moves: [u32; 7] = [
        mv("h4d8"),
        mv("h4e7"),
        mv("h4f6"),
        mv("h4g5"),
        mv("h4g3"),
        mv("h4f2"),
        mv("h4e1"),
    ];
    
    //act
    let moves = crate::generator::generate_bishop_moves(&position, sq("h4"), crate::global::COLOR_WHITE);

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
fn test_bishop_moves_other_pieces() {
    //arrange
    //white bishop h4, white pawn f6, black bishop f2
    let fen = "8/8/5P2/8/7B/8/5b2/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [u32; 3] = [
        mv("h4g5"),
        mv("h4g3"),
        mv("h4f2"),
    ];
    
    //act
    let moves = crate::generator::generate_bishop_moves(&position, sq("h4"), crate::global::COLOR_WHITE);

    //assert
    println!("expected_moves moves:");
    for move_ in expected_moves.iter() {
        println!("{}", crate::move_::Move_::get_fen(*move_));
    }

    println!("generated moves:");
    for move_ in moves.iter() {
        println!("{}", crate::move_::Move_::get_fen(*move_));
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

    let expected_moves: [u32; 2] = [
        mv("d5d6"),
        mv("d5e6"),
    ];
    
    //act
    let moves = crate::generator::generate_pawn_moves(&position, sq("d5"), crate::global::COLOR_WHITE);

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
fn test_pawn_moves_promo() {
    //arrange
    let fen = "k7/4P3/8/8/8/8/8/K7 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [u32; 4] = [
        mv("e7e8q"),
        mv("e7e8r"),
        mv("e7e8b"),
        mv("e7e8n")
    ];
    
    //act
    let moves = crate::generator::generate_pawn_moves(&position, sq("e7"), crate::global::COLOR_WHITE);

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
fn test_king_castling() {
    //arrange
    //black to move, king can go to d8 or c8 (castle)
    let fen = "r3kbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    let expected_moves: [u32; 2] = [
        mv("e8d8"),
        mv("e8c8")
    ];
    
    //act
    let moves = crate::generator::generate_king_moves(&position, sq("e8"), crate::global::COLOR_BLACK);

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

    let expected_moves: [u32; 1] = [
        mv("e8d8")
    ];
    
    //act
    let moves = crate::generator::generate_king_moves(&position, sq("e8"), crate::global::COLOR_BLACK);

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

    let expected_moves: [u32; 1] = [
        mv("e8d8")
    ];
    
    //act
    let moves = crate::generator::generate_king_moves(&position, sq("e8"), crate::global::COLOR_BLACK);

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
fn test_legal_moves() {
    //arrange
    //black to move, rook on g1, black king h8
    let fen = "7k/8/8/8/8/8/8/K5R1 b - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    //only one legal move
    let expected_moves: [u32; 1] = [
        mv("h8h7")
    ];
    
    //act
    let moves = crate::generator::generate_legal_moves(&position);

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