#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if matches!(rank, 0..=7) && matches!(file, 0..=7) {
            return Some(Self { rank, file });
        }
        None
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let pos1 = &self.position;
        let pos2 = &other.position;

        if pos1.rank == pos2.rank || pos1.file == pos2.file {
            return true;
        }

        if (pos1.rank - pos2.rank).abs() == (pos1.file - pos2.file).abs() {
            return true;
        }

        false
    }
}
