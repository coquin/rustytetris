use rustytetris::matrix::Matrix;

#[test]
fn test_from_default() {
    let rows = 3;
    let cols = 5;
    let m = Matrix::from_default(rows, cols, 0);

    assert_eq!(rows, m.rows());
    assert_eq!(cols, m.cols());
    for i in 0..rows {
        for j in 0..cols {
            assert_eq!(0, *m.get(i, j));
        }
    }
}

#[test]
fn test_from_data() {
    let rows = 2;
    let cols = 2;
    let m = Matrix::from_data(rows, cols, vec![1, 2, 3, 4]);

    assert_eq!(rows, m.rows());
    assert_eq!(cols, m.cols());
    assert_eq!(1, *m.get(0, 0));
    assert_eq!(2, *m.get(0, 1));
    assert_eq!(3, *m.get(1, 0));
    assert_eq!(4, *m.get(1, 1));
}

#[test]
#[should_panic(expected = "data does not match dimensions")]
fn test_from_data_panics_when_data_doen_not_match_dimensions() {
    let _ = Matrix::from_data(2, 2, vec![1, 2, 3]);
    let _ = Matrix::from_data(2, 2, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_get() {
    let m = Matrix::from_default(3, 5, false);

    assert_eq!(false, *m.get(1, 1));
}

#[test]
#[should_panic(expected = "row out of bounds")]
fn test_get_panics_when_row_out_of_bound() {
    let m = Matrix::from_default(3, 5, false);
    let _ = m.get(3, 1);
    let _ = m.get(4, 1);
}

#[test]
#[should_panic(expected = "col out of bounds")]
fn test_get_panics_when_col_out_of_bound() {
    let m = Matrix::from_default(3, 5, false);
    let _ = m.get(1, 5);
    let _ = m.get(1, 6);
}

#[test]
fn test_get_after_put() {
    let rows = 3;
    let cols = 5;
    let mut m = Matrix::from_default(rows, cols, 0);
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
    let mut m = Matrix::from_default(3, 5, false);
    let _ = m.put(3, 1, true);
    let _ = m.put(4, 1, true);
}

#[test]
#[should_panic(expected = "col out of bounds")]
fn test_put_panics_when_col_out_of_bound() {
    let mut m = Matrix::from_default(3, 5, false);
    let _ = m.put(1, 5, true);
    let _ = m.put(1, 6, true);
}
