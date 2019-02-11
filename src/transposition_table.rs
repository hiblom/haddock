use std::collections::HashMap;

use crate::hash_key_hasher::HashKeyBuildHasher;
use crate::move_::Move_;
use crate::outcome::Outcome;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Bound {
    Exact,
    Lower,
    Upper}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TranspositionTableEntry {
    pub horizon: i32,
    pub best_move: Move_,
    pub outcome: Outcome,
    pub bound: Bound
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

    pub fn insert(&mut self, hash_key: u64, horizon: i32, best_move: Move_, outcome: Outcome, bound: Bound) {
        match self.0.get_mut(&hash_key) {
            Some(e) => {
                if horizon >= e.horizon {
                    e.horizon = horizon;
                    e.best_move = best_move;
                    e.outcome = outcome;
                    e.bound = bound;
                }
            },
            None => {
                self.0.insert(
                    hash_key,
                    TranspositionTableEntry { horizon, best_move, outcome, bound }
                );
            }
        }
    }

    pub fn get(&self, hash_key: u64, horizon: i32) -> Option<(Move_, Outcome, Bound)> {
        if let Some(e) = self.0.get(&hash_key) {
            if e.horizon >= horizon {
                return Some((e.best_move, e.outcome, e.bound));
            }
        }
        None
    }

    pub fn get_best_move(&self, hash_hey: u64) -> Option<Move_> {
        match self.0.get(&hash_hey) {
            Some(e) => Some(e.best_move),
            None => None
        }
    }
}