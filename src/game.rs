#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }

    pub fn at(&self, row: usize, col: usize) -> Self {
        Position::new(row + self.row, col + self.col)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    pub position: Position,
}

impl Block {
    pub fn new(row: usize, col: usize) -> Self {
        Block {
            position: Position::new(row, col),
        }
    }
}
