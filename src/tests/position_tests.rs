#[test]
fn test_apply_move_e2e4() {
    //arrange
    let mut pos = crate::parser::parse_startpos().unwrap();
    let move_ = crate::move_::Move_::from_str("e2e4").unwrap();
    let move_ = pos.analyze_move(move_);

    //shoud fill enpassant square
    let ex_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";

    //act
    pos.apply_move(move_);

    //assert
    let actual_fen = crate::parser::get_position_fen(&pos);
    assert_eq!(ex_fen, actual_fen, "position not as expected\n{}", pos);

}

#[test]
fn test_apply_move_castling_e1g1() {
    //arrange
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let mut pos = crate::parser::parse_fen(&fen_parts).unwrap();
    let move_ = crate::move_::Move_::from_str("e1g1").unwrap();
    let move_ = pos.analyze_move(move_);

    //Rook should also be moved
    //white castling false
    //halfmove clock up 1
    //active color black
    let ex_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1RK1 b kq - 1 1";

    //act
    pos.apply_move(move_);

    //assert
    let actual_fen = crate::parser::get_position_fen(&pos);
    assert_eq!(ex_fen, actual_fen, "position not as expected\n{}", pos);
}

#[test]
fn test_apply_move_promo_black() {
    let fen = "4k3/8/8/8/8/8/7p/4K3 b - - 5 50";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let mut pos = crate::parser::parse_fen(&fen_parts).unwrap();

    let move_ = crate::move_::Move_::from_str("h2h1q").unwrap();
    let move_ = pos.analyze_move(move_);

    //queen on h1
    //halfmove clock back to zero
    //active color white
    //fullmovenr increased
    let ex_fen = "4k3/8/8/8/8/8/8/4K2q w - - 0 51";

    //act
    pos.apply_move(move_);

    //assert
    let actual_fen = crate::parser::get_position_fen(&pos);
    assert_eq!(ex_fen, actual_fen, "position not as expected\n{}", pos);

}

#[test]
fn test_apply_move_en_passant() {
    let fen = "4k3/8/8/3pP3/8/8/8/4K3 w - d6 5 50";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let mut pos = crate::parser::parse_fen(&fen_parts).unwrap();

    let move_ = crate::move_::Move_::from_str("e5d6").unwrap();
    let move_ = pos.analyze_move(move_);

    //white pawn on d6
    //no pawn on d5
    //black to move
    //ep square empty
    //halfmove clock 0
    let ex_fen = "4k3/8/3P4/8/8/8/8/4K3 b - - 0 50";

    //act
    pos.apply_move(move_);

    //assert
    let actual_fen = crate::parser::get_position_fen(&pos);
    assert_eq!(ex_fen, actual_fen, "position not as expected\n{}", pos);
}