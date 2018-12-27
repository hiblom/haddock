pub struct Outcome {
    pub material_value: i16,
    pub end: bool,
    pub check_mate: bool,
    pub stale_mate: bool,
    pub halfmoveclock: bool,
    pub repitition: bool
}