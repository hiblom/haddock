use crate::position::Position;

pub enum MoveResult {
    Illegal,
    Draw,
    Next(Position)
}