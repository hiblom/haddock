use std::collections::HashMap;

use crate::hash_key_hasher::HashKeyBuildHasher;
use crate::move_::Move_;
use crate::outcome::Outcome;

pub struct TranspositionTableEntry {
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

    #[allow(dead_code)]
    pub fn insert(&mut self, hash_key: u64, best_move: Option<Move_>, outcome: Option<Outcome>) {
        match self.0.get_mut(&hash_key) {
            Some(e) => {
                e.best_move = best_move;
                e.outcome = outcome;
            },
            None => {
                self.0.insert(
                    hash_key,
                    TranspositionTableEntry { best_move, outcome }
                );
            }
        }
    }

    #[allow(dead_code)]
    pub fn insert_outcome(&mut self, hash_key: u64, outcome: Option<Outcome>) {
        match self.0.get_mut(&hash_key) {
            Some(e) => {
                e.outcome = outcome;
            },
            None => {
                self.0.insert(
                    hash_key,
                    TranspositionTableEntry {
                        best_move: None,
                        outcome
                    }
                );
            }
        }
    }

    pub fn insert_best_move(&mut self, hash_key: u64, mv: Move_) {
        match self.0.get_mut(&hash_key) {
            Some(e) => {
                e.best_move = Some(mv);
            },
            None => {
                self.0.insert(
                    hash_key,
                    TranspositionTableEntry {
                        best_move: Some(mv),
                        outcome: None
                    }
                );
            }
        }
    }

    //pub fn get(&self, hash_hey: u64) -> Option<&TranspositionTableEntry> {
    //    self.0.get(&hash_hey)
    //}

    pub fn get_best_move(&self, hash_hey: u64) -> Option<Move_> {
        match self.0.get(&hash_hey) {
            Some(e) => e.best_move,
            None => None
        }
    }
}