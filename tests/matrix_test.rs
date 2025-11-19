use rustytetris::matrix::Matrix;

#[test]
fn test_create() {
    let m = Matrix::new(3, 5, false);

    assert_eq!(3, m.rows());
    assert_eq!(5, m.cols());
}
