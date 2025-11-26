use rustytetris::game::*;
use rustytetris::gamefield::GameField;
use rustytetris::tetromino::Tetromino;

#[test]
fn test_create() {
    let gf = GameField::new(2, 2);
    assert_eq!(0, gf.blocks().len());
}

#[test]
fn test_put_one() {
    let mut gf = GameField::new(2, 2);
    let o = Tetromino::o();
    gf.put(0, 0, o);

    let actual = gf.blocks();
    let expected = vec![
        Block::new(0, 0),
        Block::new(0, 1),
        Block::new(1, 0),
        Block::new(1, 1),
    ];
    assert_eq!(expected, actual);
}

#[test]
fn test_put_two() {
    let mut gf = GameField::new(2, 4);
    let o1 = Tetromino::o();
    let o2 = Tetromino::o();
    gf.put(0, 0, o1);
    gf.put(0, 2, o2);

    let actual = gf.blocks();
    let expected = vec![
        Block::new(0, 0),
        Block::new(0, 1),
        Block::new(1, 0),
        Block::new(1, 1),
        Block::new(0, 2),
        Block::new(0, 3),
        Block::new(1, 2),
        Block::new(1, 3),
    ];
    assert_eq!(expected, actual);
}
