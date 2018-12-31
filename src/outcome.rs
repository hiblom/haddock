use std::cmp::Ordering;

use crate::global;

#[derive(Copy, Clone, Debug)]
pub enum Outcome {
    Undecided(i16),
    WhiteIsMate(u16),
    BlackIsMate(u16),
    DrawByStalemate,
    DrawByHalfmoveclock,
    DrawByRepetition,
    DrawByInsufficientMaterial
}

impl Outcome {
    pub fn score(&self) -> i16 {
        match self {
            Outcome::Undecided(material_value) => *material_value,
            Outcome::WhiteIsMate(_) => -10000,
            Outcome::BlackIsMate(_) => 10000,
            _ => 0
        }
    }

    pub fn to_uci_score(&self, active_color: u8) -> String {
        let mult = match active_color {
            global::COLOR_WHITE => -1,
            _ => 1
        };

        match self {
            Outcome::Undecided(material_value) => format!("cp {}", *material_value),
            Outcome::WhiteIsMate(n) => format!("mate {}", mult * (*n as i16)),
            Outcome::BlackIsMate(n) => format!("mate {}", - mult * (*n as i16)),
            _ => "cp 0".to_string()
        }
    }

    pub fn end(&self) -> bool {
        match self {
            Outcome::Undecided(_) => false,
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