#[test]
pub fn test_hash_startpos() {
    //arrange

    //act
    let position = crate::parser::parse_startpos().unwrap();

    //assert
    let actual_hash = position.get_hash();
    assert_eq!(14668778111269048869u64, actual_hash, "hash not as expected");
}

#[test]
pub fn test_same_transposition_hash() {
    //arrange
    let mut position1 = crate::parser::parse_startpos().unwrap();
    let mut position2 = crate::parser::parse_startpos().unwrap();

    //NOTE
    //if we do this with sequence e2e4-e7e5-d2d4 <> d2d4-d7e5-d2d4 the hashes won't be equal, because of the EP square
    let mv_e2e3 = crate::move_::Move_::from_str("e2e3").unwrap();
    let mv_e7e5 = crate::move_::Move_::from_str("e7e5").unwrap();
    let mv_d2d3 = crate::move_::Move_::from_str("d2d3").unwrap();

    //act
    let e2e3 = position1.analyze_move(mv_e2e3);
    position1.apply_move(e2e3);
    let e7e5 = position1.analyze_move(mv_e7e5);
    position1.apply_move(e7e5);
    let d2d3 = position1.analyze_move(mv_d2d3);
    position1.apply_move(d2d3);

    let d2d3 = position2.analyze_move(mv_d2d3);
    position2.apply_move(d2d3);
    let e7e5 = position2.analyze_move(mv_e7e5);
    position2.apply_move(e7e5);
    let e2e3 = position2.analyze_move(mv_e2e3);
    position2.apply_move(e2e3);

    //assert
    let hash1 = position1.get_hash();
    let hash2 = position2.get_hash();
    assert_eq!(hash1, hash2, "hashes should be equal");
}

#[test]
pub fn test_ep_square_hash_different() {
    //arrange
    let mut position1 = crate::parser::parse_startpos().unwrap();
    let mut position2 = crate::parser::parse_startpos().unwrap();

    //NOTE
    let mv_e2e4 = crate::move_::Move_::from_str("e2e4").unwrap();
    let mv_e7e5 = crate::move_::Move_::from_str("e7e5").unwrap();
    let mv_d2d4 = crate::move_::Move_::from_str("d2d4").unwrap();

    //act
    let e2e4 = position1.analyze_move(mv_e2e4);
    position1.apply_move(e2e4);
    let e7e5 = position1.analyze_move(mv_e7e5);
    position1.apply_move(e7e5);
    let d2d4 = position1.analyze_move(mv_d2d4);
    position1.apply_move(d2d4);

    let d2d4 = position2.analyze_move(mv_d2d4);
    position2.apply_move(d2d4);
    let e7e5 = position2.analyze_move(mv_e7e5);
    position2.apply_move(e7e5);
    let e2e4 = position2.analyze_move(mv_e2e4);
    position2.apply_move(e2e4);

    //assert
    let hash1 = position1.get_hash();
    let hash2 = position2.get_hash();
    assert_ne!(hash1, hash2, "hashes should not be equal");
}



