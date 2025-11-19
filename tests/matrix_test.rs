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
