use std::collections::HashMap;

use crate::hash_key_hasher::HashKeyBuildHasher;
use crate::move_::Move_;
use crate::outcome::Outcome;

pub struct TranspositionTableEntry {
    pub best_move: Move_,
    pub outcome: Option<Outcome>
}

pub struct TranspositionTable(HashMap<u64, TranspositionTableEntry, HashKeyBuildHasher>);

impl TranspositionTable {
    pub fn new() -> Self {
        TranspositionTable(HashMap::default())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, hash_key: u64, entry: TranspositionTableEntry) {
        self.0.insert(hash_key, entry);
    }

    pub fn get(&self, hash_hey: u64) -> Option<&TranspositionTableEntry> {
        self.0.get(&hash_hey)
    }
}