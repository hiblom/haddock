use std::collections::HashMap;

use crate::outcome::Outcome;
use crate::move_::Move_;

pub struct Tree {
    pub best_score: Option<Outcome>,
    pub best_move: Option<Move_>,
    pub sub_trees: Option<HashMap<Move_, Tree>>
}