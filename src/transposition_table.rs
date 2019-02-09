use std::collections::HashMap;

use crate::hash_key_hasher::HashKeyBuildHasher;
use crate::move_::Move_;
use crate::outcome::Outcome;

pub struct TranspositionTableEntry {
    pub horizon: i32,
    pub best_move: Option<Move_>,
    pub outcome: Option<Outcome>
}

pub struct TranspositionTable(HashMap<u64, TranspositionTableEntry, HashKeyBuildHasher>);

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable(HashMap::default())
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, hash_key: u64, horizon: i32, best_move: Option<Move_>, outcome: Option<Outcome>) {
        match self.0.get_mut(&hash_key) {
            Some(e) => {
                if horizon >= e.horizon {
                    e.horizon = horizon;
                    e.best_move = best_move;
                    e.outcome = outcome;
                }
            },
            None => {
                self.0.insert(
                    hash_key,
                    TranspositionTableEntry { horizon, best_move, outcome }
                );
            }
        }
    }

    #[allow(dead_code)]
    pub fn insert_outcome(&mut self, hash_key: u64, horizon: i32, outcome: Option<Outcome>) {
        match self.0.get_mut(&hash_key) {
            Some(e) => {
                if horizon >= e.horizon {
                    e.horizon = horizon;
                    e.outcome = outcome;
                }
            },
            None => {
                self.0.insert(
                    hash_key,
                    TranspositionTableEntry {
                        horizon,
                        best_move: None,
                        outcome
                    }
                );
            }
        }
    }

    #[allow(dead_code)]
    pub fn insert_best_move(&mut self, hash_key: u64, horizon: i32, mv: Move_) {
        match self.0.get_mut(&hash_key) {
            Some(e) => {
                if horizon >= e.horizon {
                    e.horizon = horizon;
                    e.best_move = Some(mv);
                }
            },
            None => {
                self.0.insert(
                    hash_key,
                    TranspositionTableEntry {
                        horizon,
                        best_move: Some(mv),
                        outcome: None
                    }
                );
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_outcome(&self, hash_key: u64, horizon: i32) -> Option<Outcome> {
        match self.0.get(&hash_key) {
            Some(e) => {
                if e.horizon >= horizon {
                    return e.outcome;
                } else {
                    return None;
                }
            },
            None => None
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, hash_key: u64, horizon: i32) -> Option<(Move_, Outcome)> {
        match self.0.get(&hash_key) {
            Some(e) => {
                if e.horizon >= horizon  && e.best_move.is_some() && e.outcome.is_some() {
                    return Some((e.best_move.unwrap(), e.outcome.unwrap()));
                }
            },
            None => {
                return None;
            }
        }
        None
    }

    pub fn get_best_move(&self, hash_hey: u64) -> Option<Move_> {
        match self.0.get(&hash_hey) {
            Some(e) => e.best_move,
            None => None
        }
    }
}