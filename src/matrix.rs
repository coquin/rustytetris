pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn from_default(rows: usize, cols: usize, value: T) -> Self {
        Self {
            rows,
            cols,
            data: vec![value; rows * cols],
        }
    }

    pub fn from_data(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert!(rows * cols == data.len(), "data does not match dimensions");
        Self {
            rows: rows,
            cols: cols,
            data: data,
        }
    }

    pub fn put(&mut self, row: usize, col: usize, value: T) {
        let idx = self.index(row, col);
        self.data[idx] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        let idx = self.index(row, col);
        return &self.data[idx];
    }

    pub fn rows(&self) -> usize {
        return self.rows;
    }

    pub fn cols(&self) -> usize {
        return self.cols;
    }

    fn index(&self, row: usize, col: usize) -> usize {
        assert!(row < self.rows, "row out of bounds");
        assert!(col < self.cols, "col out of bounds");
        row * self.cols + col
    }
}
