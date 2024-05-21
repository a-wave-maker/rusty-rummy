#[derive(Clone)]
pub struct Piece {
    pub color: u8,
    pub value: u8,
}

pub struct Sequence {
    pub pieces: Vec<Piece>,
}