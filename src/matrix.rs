pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(width: usize, height: usize, value: T) -> Self {
        Self {
            rows: width,
            cols: height,
            data: vec![value; width * height],
        }
    }

    pub fn put(&mut self, row: usize, col: usize, value: T) {
        // self.data[row * self.cols + col] = value;
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
