#[test]
pub fn test_get_fen_e2() {
    //arrange
    let square = crate::square::Square::new(12);
    
    //act
    let square_string =square.to_fen();

    //assert
    assert_eq!("e2", square_string, "Unexpected square string");
}

#[test]
pub fn test_from_str() {
    //arrange
    let fen = "e2";
    
    //act
    let square = crate::square::Square::from_str(fen).unwrap();

    //assert
    assert_eq!(12, square.to_u32(), "Unexpected square");
}