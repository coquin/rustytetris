use crate::tetromino::Tetromino;

pub struct GameField {
    blocks: Vec<Block>,
}

impl GameField {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn put(&mut self, row: usize, col: usize, t: Tetromino) {
        self.blocks = vec![
            Block::new(0, 0),
            Block::new(0, 1),
            Block::new(1, 0),
            Block::new(1, 1),
        ]
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Block {
    pub position: Position,
}

impl Block {
    pub fn new(row: usize, col: usize) -> Self {
        Block {
            position: Position { row, col },
        }
    }
}
