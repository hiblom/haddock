
#[test]
pub fn test_parse_empty_fen() {
    //arrange
    let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();

    //act
    let position = crate::parser::parse_fen(&fen_parts).unwrap();

    //assert
    let actual_fen = crate::parser::get_position_fen(&position);
    assert_eq!(fen, actual_fen, "position not as expected");
}

#[test]
fn test_parse_startpos() {
    //act
    let position = crate::parser::parse_startpos().unwrap();

    //assert
    let actual_fen = crate::parser::get_position_fen(&position);
    assert_eq!(crate::global::FEN_STARTPOS, actual_fen, "position not as expected");
}

