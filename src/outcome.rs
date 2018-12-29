use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
pub enum Outcome {
    Undecided(i16),
    WhiteIsMate,
    BlackIsMate,
    DrawByStalemate,
    DrawByHalfmoveclock,
    DrawByRepetition
}

impl Outcome {
    pub fn score(&self) -> i16 {
        match self {
            Outcome::Undecided(material_value) => *material_value,
            Outcome::WhiteIsMate => -10000,
            Outcome::BlackIsMate => 10000,
            _ => 0
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