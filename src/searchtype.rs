#[derive(Clone, Copy)]
pub enum SearchType {
    Infinite,
    CTime(u64, u64, u64, u64),
    Depth(u64),
    Nodes(u64),
    MoveTime(u64)
}