use rustytetris::matrix::Matrix;

#[test]
fn test_create() {
    let m = Matrix::new(3, 5, false);

    assert_eq!(3, m.rows());
    assert_eq!(5, m.cols());
}

#[test]
fn test_get() {
    let m = Matrix::new(3, 5, false);

    assert_eq!(false, *m.get(1, 1));
}

#[test]
#[should_panic(expected = "row out of bounds")]
fn test_get_panics_when_row_out_of_bound() {
    let m = Matrix::new(3, 5, false);
    let _ = m.get(3, 1);
    let _ = m.get(4, 1);
}

#[test]
#[should_panic(expected = "col out of bounds")]
fn test_get_panics_when_col_out_of_bound() {
    let m = Matrix::new(3, 5, false);
    let _ = m.get(1, 5);
    let _ = m.get(1, 6);
}

#[test]
fn test_get_after_put() {
    let rows = 3;
    let cols = 5;
    let mut m = Matrix::new(rows, cols, 0);
    let mut v = 1;

    for i in 0..rows {
        for j in 0..cols {
            m.put(i, j, v);
            v += 1;
        }
    }

    v = 1;

    for i in 0..rows {
        for j in 0..cols {
            assert_eq!(v, *m.get(i, j));
            v += 1;
        }
    }
}

#[test]
#[should_panic(expected = "row out of bounds")]
fn test_put_panics_when_row_out_of_bound() {
    let mut m = Matrix::new(3, 5, false);
    let _ = m.put(3, 1, true);
    let _ = m.put(4, 1, true);
}

#[test]
#[should_panic(expected = "col out of bounds")]
fn test_put_panics_when_col_out_of_bound() {
    let mut m = Matrix::new(3, 5, false);
    let _ = m.put(1, 5, true);
    let _ = m.put(1, 6, true);
}
