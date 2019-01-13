use std::cmp::Ordering;

use crate::global;

#[derive(Copy, Clone, Debug)]
pub enum Outcome {
    Illegal(i32),
    Undecided(i32, i32),
    #[allow(dead_code)] WhiteIsMate(i32),
    #[allow(dead_code)] BlackIsMate(i32),
    #[allow(dead_code)] DrawByStalemate(i32),
    DrawByHalfmoveclock(i32),
    #[allow(dead_code)] DrawByRepetition(i32),
    #[allow(dead_code)] DrawByInsufficientMaterial(i32)
}

impl Outcome {
    pub fn score(&self) -> i32 {
        match self {
            Outcome::Undecided(_, material_value) => *material_value,
            Outcome::WhiteIsMate(d) => -20000 + d, // to look for quicker mate
            Outcome::BlackIsMate(d) => 20000 - d,
            _ => 0
        }
    }

    pub fn to_uci_score(&self, active_color: u8) -> String {
        let mult = match active_color {
            global::COLOR_WHITE => 1,
            _ => - 1
        };

        match self {
            Outcome::Undecided(_, material_value) => format!("cp {}", mult * *material_value),
            Outcome::BlackIsMate(depth) => format!("mate {}", mult * (*depth + 1) / 2), //plies to moves
            Outcome::WhiteIsMate(depth) => format!("mate {}", - mult * (*depth + 1) / 2), //plies to moves
            _ => "cp 0".to_string()
        }
    }

    pub fn end(&self) -> bool {
        match self {
            Outcome::Undecided(_, _) => false,
            _ => true
        }
    }
}

impl Ord for Outcome {
    fn cmp(&self, other: &Outcome) -> Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for Outcome {
    fn partial_cmp(&self, other: &Outcome) -> Option<Ordering> {
        Some(self.score().cmp(&other.score()))
    }
}

impl Eq for Outcome {}

impl PartialEq for Outcome {
    fn eq(&self, other: &Outcome) -> bool {
        self.score() == other.score()
    }
}