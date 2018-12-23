#[test]
pub fn test_get_fen_e2e4() {
    //arrange
    let mv: u32 = 12 * 256 + 28;

    //act
    let mv_string = crate::move_::Move_::get_fen(mv);

    //assert
    assert_eq!("e2e4", mv_string, "Unexpected move string");
}

#[test]
pub fn test_from_str() {
    //arrange
    let mv_string = "e2e4";
    let expected_mv: u32 = 12 * 256 + 28;

    //act
    let mv: u32 = crate::move_::Move_::from_str(mv_string).unwrap();

    //assert
    assert_eq!(expected_mv, mv, "Unexpected move from string");
}

#[test]
pub fn test_from_to_str_promo() {
    //arrange
    let mv_string = "e7e8q";

    //act
    let mv: u32 = crate::move_::Move_::from_str(mv_string).unwrap();
    let mv_str_back = crate::move_::Move_::get_fen(mv);

    //assert
    assert_eq!(mv_string, mv_str_back, "Unexpected move from string");
}