use crate::*;

#[test]
fn test_deck_from_input() {
    let mut deck = Deck::from_input("9 2 6 3 1");
    assert_eq!(deck.cards.pop_back().unwrap(), 1);
    assert_eq!(deck.cards.pop_back().unwrap(), 3);
    assert_eq!(deck.cards.pop_back().unwrap(), 6);
    assert_eq!(deck.cards.pop_back().unwrap(), 2);
    assert_eq!(deck.cards.pop_back().unwrap(), 9);
    assert!(deck.cards.is_empty());
}

#[test]
fn test_deck_next_card() {
    let mut deck = Deck::from_input("9 2 6 3 1");
    assert_eq!(deck.next_card(), 9);
    assert_eq!(deck.next_card(), 2);
    assert_eq!(deck.next_card(), 6);
    assert_eq!(deck.next_card(), 3);
    assert_eq!(deck.next_card(), 1);
    assert!(deck.cards.is_empty());
}

#[test]
fn test_deck_add_to_bottom() {
    let mut deck = Deck::from_input("9");
    assert_eq!(deck.next_card(), 9);

    deck.add_to_bottom(10);
    deck.add_to_bottom(50);


    assert_eq!(deck.next_card(), 10);
    assert_eq!(deck.next_card(), 50);

    assert!(deck.cards.is_empty());
}

#[test]
fn test_deck_add_winning_cards() {
    let mut deck = Deck::from_input("9");
    deck.add_winning_cards(10, 50);

    assert_eq!(deck.next_card(), 9);
    assert_eq!(deck.next_card(), 10);
    assert_eq!(deck.next_card(), 50);
}

#[test]
fn test_play_game() {
    let mut player1_deck = Deck::from_input("9 2 6 3 1");
    let mut player2_deck = Deck::from_input("5 8 4 7 10");

    play_game_of_combat(&mut player1_deck, &mut player2_deck);
}

#[test]
fn test_deck_calculate_score() {
    let deck = Deck::from_input("3 2 10 6 8 5 9 4 7 1");
    assert_eq!(deck.calculate_score(), 306);
}

#[test]
fn test_recursive_combat_has_deck_been_seen_before() {
    let mut game = RecursiveCombatGame::from_inputs("infinite game", "43 19", "2 29 14");
    assert!(!game.has_deck_been_seen_before(Player::One, &Deck::from_input("43 19")));
    assert!(!game.has_deck_been_seen_before(Player::Two, &Deck::from_input("2 29 14")));
    game.save_decks_configuration();
    assert!(game.has_deck_been_seen_before(Player::One, &Deck::from_input("43 19")));
    assert!(game.has_deck_been_seen_before(Player::Two, &Deck::from_input("2 29 14")));
}

#[test]
fn test_recursive_combat_play_till_winner_infinite_rule() {
    let mut game = RecursiveCombatGame::from_inputs("infinite game", "43 19", "2 29 14");
    assert_eq!(game.play_till_winner(), Player::One);
}

#[test]
fn test_deck_create_deck_from_top_n_cards() {
    let deck = Deck::from_input("9 8 5 2");
    assert_eq!(deck.create_deck_from_top_n_cards(4), Deck::from_input("9 8 5 2"));

    let deck = Deck::from_input("10 1 7 6");
    assert_eq!(deck.create_deck_from_top_n_cards(3), Deck::from_input("10 1 7"));
}

#[test]
fn test_recursive_combat_play_till_winner_with_sub_games() {
    let mut game = RecursiveCombatGame::from_inputs(
        "sub-games test",
        "9 2 6 3 1",
        "5 8 4 7 10"
    );
    assert_eq!(game.play_till_winner(), Player::Two);
}