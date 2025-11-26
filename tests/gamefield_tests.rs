use rustytetris::gamefield::{Block, GameField};
use rustytetris::tetromino::Tetromino;

#[test]
fn test_create() {
    let gf = GameField::new(2, 2);
    assert_eq!(0, gf.blocks().len());
}

#[test]
fn test_put() {
    let mut gf = GameField::new(2, 2);
    let o = Tetromino::o();
    gf.put(1, 1, o);

    let actual = gf.blocks();
    let expected = vec![
        Block::new(0, 0),
        Block::new(0, 1),
        Block::new(1, 0),
        Block::new(1, 1),
    ];
    assert_eq!(expected, actual);
}
