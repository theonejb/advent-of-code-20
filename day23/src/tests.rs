use crate::*;
use itertools::{assert_equal, Itertools};

fn get_sample_game() -> Game {
    let inputs = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    Game::from_inputs(&inputs)
}

#[test]
fn test_game_from_inputs() {
    let game = get_sample_game();

    assert_equal(game.current_arrangement.iter(), [8, 9, 1, 2, 5, 4, 6, 7, 3].iter());

    assert_eq!(game.min_cup, 1);
    assert_eq!(game.max_cup, 9);
}

#[test]
fn test_game_pick_cups() {
    let mut game = get_sample_game();
    assert_equal(game.pick_cups(3), vec![8, 9, 1]);
}

#[test]
fn test_game_get_designated_cup() {
    let mut game = get_sample_game();

    assert_eq!(game.get_designated_cup(&vec![8, 9, 1]), 2);
}

#[test]
fn test_game_place_cups() {
    let mut game = get_sample_game();
    let picked_cups = game.pick_cups(3);
    let designated_cup = game.get_designated_cup(&picked_cups);
    game.place_cups(designated_cup, &picked_cups);

    assert_equal(game.current_arrangement.iter(), vec![2, 8,  9,  1,  5,  4,  6,  7, 3].iter())
}

#[test]
fn test_game_select_next_current_cup() {
    let mut game = get_sample_game();
    game.select_next_current_cup();
    assert_eq!(game.get_current_cup(), 8);
}