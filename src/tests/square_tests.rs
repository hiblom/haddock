#[test]
pub fn test_get_fen_e2() {
    //arrange
    let square = 12;
    
    //act
    let square_string = crate::square::Square::get_fen(square);

    //assert
    assert_eq!("e2", square_string, "Unexpected square string");
}

pub fn test_from_str() {
    //arrange
    let fen = "e2";
    
    //act
    let square: u8 = crate::square::Square::from_str(fen).unwrap();

    //assert
    assert_eq!(12, square, "Unexpected square");
}