
#[test]
pub fn test_parse_empty_fen() {
    //arrange
    let fen = "8/8/8/8/8/8/8/8 w - - 0 1";
    let fen_parts = fen.split(" ").collect::<Vec<&str>>();

    let expected_pieces: [u8; 64] = [0; 64];
    let expected_castling: [bool; 4] = [false; 4];

    //act
    let position = crate::parser::parse_fen(&fen_parts);

    //assert
    match position {
        Some(pos) => {
            assert_eq!(
                &expected_pieces[..],
                &pos.pieces[..],
                "Pieces not as expected"
            );
            assert_eq!(0, pos.active_color, "Color not as expected");
            assert_eq!(
                &expected_castling, &pos.castling_status,
                "Castling status not as expected"
            );
            assert!(pos.enpassant_square.is_none(), "Enpassant not as expexted");
            assert_eq!(0, pos.halfmoveclock, "Half move clock not as expected");
            assert_eq!(1, pos.fullmovenumber, "Full move number not as expected");
        }
        None => assert!(false, "Position is empty"),
    }
}

#[test]
fn test_parse_startpos() {
    //arrange
    //note: board is upside down, white (even) pieces at top
    let expected_pieces: [u8; 64] = [
        6, 10, 8, 4, 2, 8, 10, 6, 
        12, 12, 12, 12, 12, 12, 12, 12, 
        0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 
        0, 0, 0, 0, 0, 0, 0, 0, 
        13, 13, 13, 13, 13, 13, 13, 13, 
        7, 11, 9, 5, 3, 9, 11, 7
    ];
    let expected_castling: [bool; 4] = [true; 4];

    //act
    let position = crate::parser::parse_startpos();

    //assert
    match position {
        Some(pos) => {
            assert_eq!(
                &expected_pieces[..],
                &pos.pieces[..],
                "Pieces not as expected"
            );
            assert_eq!(
                crate::global::COLOR_WHITE,
                pos.active_color,
                "Color not as expected"
            );
            assert_eq!(
                &expected_castling, &pos.castling_status,
                "Castling status not as expected"
            );
            assert!(pos.enpassant_square.is_none(), "Enpassant not as expexted");
            assert_eq!(0, pos.halfmoveclock, "Half move clock not as expected");
            assert_eq!(1, pos.fullmovenumber, "Full move number not as expected");
        }
        None => assert!(false, "Position is empty"),
    }
}

