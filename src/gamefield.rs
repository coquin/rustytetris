use crate::game::*;
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
        for p in t.positions {
            self.blocks.push(Block {
                position: p.at(row, col),
            });
        }
    }
}
