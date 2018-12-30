fn mv(mv_str: &str) -> u32 {
    crate::move_::Move_::from_str(mv_str).unwrap()
}

fn sq(sq_str: &str) -> u8 {
    crate::square::Square::from_str(sq_str).unwrap()
}

#[test]
fn test_is_check_startpos() {
    //arrange
    let position = crate::parser::parse_startpos().unwrap();
    
    //act
    let check = crate::evaluation::is_check(&position, position.active_color);

    //assert
    assert_eq!(
        false,
        check,
        "Check not as expected"
    );

}

#[test]
fn test_is_check_by_queen() {
    //arrange
    //black queen d8, white king d1
    let fen = "k2q4/8/8/8/8/8/8/3K4 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();
    
    //act
    let check = crate::evaluation::is_check(&position, position.active_color);

    //assert
    assert_eq!(
        true,
        check,
        "Check not as expected"
    );
}

#[test]
fn test_is_check_by_rook() {
    //arrange
    //white rook h1, black king h8
    let fen = "7k/8/8/8/8/8/8/K6R b - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();
    
    //act
    let check = crate::evaluation::is_check(&position, position.active_color);

    //assert
    assert_eq!(
        true,
        check,
        "Check not as expected"
    );
}

#[test]
fn test_is_check_by_bishop() {
    //arrange
    //white bishop b2, black king h8
    let fen = "7k/8/8/8/8/8/1B6/K7 b - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();
    
    //act
    let check = crate::evaluation::is_check(&position, position.active_color);

    //assert
    assert_eq!(
        true,
        check,
        "Check not as expected"
    );
}

#[test]
fn test_is_check_by_knight() {
    //arrange
    //white knight d5, black king f6
    let fen = "8/8/5k2/3N4/8/8/8/K7 b - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();
    
    //act
    let check = crate::evaluation::is_check(&position, position.active_color);

    //assert
    assert_eq!(
        true,
        check,
        "Check not as expected"
    );
}

#[test]
fn test_is_check_by_pawn() {
    //arrange
    //white pawn g5, black king f6
    let fen = "8/8/5k2/6P1/8/8/8/K7 b - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();
    let position = crate::parser::parse_fen(&fen_parts).unwrap();
    
    //act
    let check = crate::evaluation::is_check(&position, position.active_color);

    //assert
    assert_eq!(
        true,
        check,
        "Check not as expected"
    );
}



#[test]
fn test_fools_mate() {
    //arrange
    let mut position = crate::parser::parse_startpos().unwrap();
    
    position.apply_move(mv("f2f3"));
    position.apply_move(mv("e7e5"));

    position.apply_move(mv("g2g4"));
    position.apply_move(mv("d8h4"));

    //act
    let score = crate::evaluation::evaluate(&position);

    //assert
    assert_eq!(
        crate::outcome::Outcome::WhiteIsMate(0),
        score,
        "Check mate not as expected"
    );
}