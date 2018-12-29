use std::collections::HashMap;

use crate::outcome::Outcome;

pub struct Tree {
    pub start_score: Outcome,
    pub best_score: Option<Outcome>,
    pub best_move: Option<u32>,
    pub sub_trees: Option<HashMap<u32, Tree>>
}