use crate::*;

#[test]
fn test_next_number() {
    let mut game = MemoryGame::new(&[0,3,6]);

    for expected_number in [0, 3, 6, 0, 3, 3, 1, 0, 4, 0].iter() {
        let next_number = game.next_number();
        // println!("Turn: {}; Number: {}", turn+1, next_number);
        assert_eq!(next_number, *expected_number);
    }
}

#[test]
fn test_nth() {
    let mut game = MemoryGame::new(&[0, 3, 6]);
    assert_eq!(game.nth(2020), 436);

    let mut game = MemoryGame::new(&[1, 3, 2]);
    assert_eq!(game.nth(2020), 1);

    let mut game = MemoryGame::new(&[2, 1, 3]);
    assert_eq!(game.nth(2020), 10);

    let mut game = MemoryGame::new(&[1, 2, 3]);
    assert_eq!(game.nth(2020), 27);

    let mut game = MemoryGame::new(&[2, 3, 1]);
    assert_eq!(game.nth(2020), 78);

    let mut game = MemoryGame::new(&[3, 2, 1]);
    assert_eq!(game.nth(2020), 438);

    let mut game = MemoryGame::new(&[3, 1, 2]);
    assert_eq!(game.nth(2020), 1836);
}

#[test]
fn test_part_2_nth() {
    let mut game = MemoryGame::new(&[0, 3, 6]);
    assert_eq!(game.nth(30_000_000), 175594);
    //
    // let mut game = MemoryGame::new(&[1, 3, 2]);
    // assert_eq!(game.nth(30000000), 2578);
    //
    // let mut game = MemoryGame::new(&[2, 1, 3]);
    // assert_eq!(game.nth(30000000), 3544142);
    //
    // let mut game = MemoryGame::new(&[1, 2, 3]);
    // assert_eq!(game.nth(30000000), 261214);
    //
    // let mut game = MemoryGame::new(&[2, 3, 1]);
    // assert_eq!(game.nth(30000000), 6895259);
    //
    // let mut game = MemoryGame::new(&[3, 2, 1]);
    // assert_eq!(game.nth(30000000), 18);
    //
    // let mut game = MemoryGame::new(&[3, 1, 2]);
    // assert_eq!(game.nth(30000000), 362);
}