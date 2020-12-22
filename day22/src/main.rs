use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Result};

mod tests;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Deck {
    cards: VecDeque<u8>
}

impl Deck {
    pub fn from_input(input: &str) -> Deck {
        let cards_input = input.split_whitespace();
        let mut cards = VecDeque::new();

        for card in cards_input {
            let card = card.parse::<u8>().unwrap();
            cards.push_back(card);
        }

        Deck { cards }
    }

    pub fn create_deck_from_top_n_cards(&self, n: usize) -> Deck {
        let mut new_cards = self.cards.clone();
        new_cards.truncate(n);

        Deck {
            cards: new_cards
        }
    }

    pub fn next_card(&mut self) -> u8 {
        self.cards.pop_front().unwrap()
    }

    pub fn add_to_bottom(&mut self, card: u8) {
        self.cards.push_back(card);
    }

    pub fn add_winning_cards(&mut self, card1: u8, card2: u8) {
        self.add_to_bottom(card1);
        self.add_to_bottom(card2);
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn calculate_score(&self) -> u32 {
        let mut score = 0;

        for (place, card) in self.cards.iter().rev().enumerate() {
            score += (place + 1) as u32 * *card as u32;
        }

        score
    }

    pub fn number_of_cards(&self) -> usize {
        self.cards.len()
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", itertools::join(self.cards.iter(), " "))
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Player { One, Two }

#[derive(Debug)]
struct RecursiveCombatGame {
    name: String,

    player1_deck: Deck,
    player1_previous_decks: Vec<Deck>,

    player2_deck: Deck,
    player2_previous_decks: Vec<Deck>,
}

impl RecursiveCombatGame {
    pub fn from_inputs(name: &str, player1_input: &str, player2_input: &str) -> RecursiveCombatGame {
        RecursiveCombatGame {
            name: String::from(name),

            player1_deck: Deck::from_input(player1_input),
            player1_previous_decks: vec![],

            player2_deck: Deck::from_input(player2_input),
            player2_previous_decks: vec![],
        }
    }

    pub fn from_decks(name: &str, player1_deck: Deck, player2_deck: Deck) -> RecursiveCombatGame {
        RecursiveCombatGame {
            name: String::from(name),

            player1_deck,
            player1_previous_decks: vec![],

            player2_deck,
            player2_previous_decks: vec![]
        }
    }

    pub fn save_decks_configuration(&mut self) {
        self.player1_previous_decks.push(
            self.player1_deck.clone()
        );
        self.player2_previous_decks.push(
            self.player2_deck.clone()
        );
    }

    pub fn has_deck_been_seen_before(&self, player: Player, deck: &Deck) -> bool {
        (match player {
            Player::One => &self.player1_previous_decks,
            Player::Two => &self.player2_previous_decks
        }).contains(deck)
    }

    fn infinite_game_rule_triggered(&self) -> bool {
        self.has_deck_been_seen_before(Player::One, &self.player1_deck) ||
            self.has_deck_been_seen_before(Player::Two, &self.player2_deck)
    }

    pub fn play_till_winner(&mut self) -> Player {
        // println!("== Game {} ==", self.name);

        let mut round = 0;
        let winner;

        loop {
            round += 1;

            // println!("-- Round {} (Game {})--", round, self.name);
            // println!("Player 1's deck: {}", self.player1_deck);
            // println!("Player 2's deck: {}", self.player2_deck);

            if self.infinite_game_rule_triggered() {
                // println!("Player 1 wins by infinite game rule.");
                winner = Player::One;
                break;
            }

            if self.player1_deck.is_empty() {
                winner = Player::Two;
                break;
            } else if self.player2_deck.is_empty() {
                winner = Player::One;
                break;
            }

            self.save_decks_configuration();

            let c1 = self.player1_deck.next_card();
            let c2 = self.player2_deck.next_card();

            // println!("Player 1 plays: {}", c1);
            // println!("Player 2 plays: {}", c2);

            let mut round_winner = None;

            // Recursive game time
            if (self.player1_deck.number_of_cards() as u32 >= c1 as u32) &&
                (self.player2_deck.number_of_cards() as u32 >= c2 as u32) {
                // println!("Playing a sub-game to determine the winner...");
                let player1_deck = self.player1_deck.create_deck_from_top_n_cards(c1 as usize);
                let player2_deck = self.player2_deck.create_deck_from_top_n_cards(c2 as usize);
                let mut game = RecursiveCombatGame::from_decks(
                    format!("{} sub-game (started at round {})", self.name, round).as_str(),
                    player1_deck,
                    player2_deck
                );

                round_winner = Some(game.play_till_winner());
            } else {
                if c1 > c2 {
                    round_winner = Some(Player::One);
                } else {
                    round_winner = Some(Player::Two);
                }
            }

            if let Some(winner) = round_winner {
                if winner == Player::One {
                    // println!("Player 1 wins this round.");
                    self.player1_deck.add_winning_cards(c1, c2);
                } else {
                    // println!("Player 2 wins this round.");
                    self.player2_deck.add_winning_cards(c2, c1);
                }
            }
        }

        // println!("== Post-game results ==");
        // println!("Player 1's deck: {}", self.player1_deck);
        // println!("Player 2's deck: {}", self.player2_deck);

        winner
    }
}

fn play_game_of_combat(player1_deck: &mut Deck, player2_deck: &mut Deck) {
    let mut round = 0;

    while !player1_deck.is_empty() && !player2_deck.is_empty() {
        round += 1;

        // println!("-- Round {} --", round);
        // println!("Player 1's deck: {}", player1_deck);
        // println!("Player 2's deck: {}", player2_deck);

        let c1 = player1_deck.next_card();
        let c2 = player2_deck.next_card();

        // println!("Player 1 plays: {}", c1);
        // println!("Player 2 plays: {}", c2);

        if c1 > c2 {
            // println!("Player 1 wins this round.");
            player1_deck.add_winning_cards(c1, c2);
        } else {
            // println!("Player 2 wins this round.");
            player2_deck.add_winning_cards(c2, c1);
        }

        // println!()
    }

    // println!("== Post-game results ==");
    // println!("Player 1's deck: {}", player1_deck);
    // println!("Player 2's deck: {}", player2_deck);
}

fn get_inputs() -> (String, String) {
    (
        String::from("42 29 12 40 47 26 11 39 41 13 8 50 44 33 5 27 10 25 17 1 28 22 6 32 35"),
        String::from("19 34 38 21 43 14 23 46 16 3 36 31 37 45 30 15 49 48 24 9 2 18 4 7 20")
    )
}

fn main() {
    let (player1_input, player2_input) = get_inputs();
    let mut player1_deck = Deck::from_input(&player1_input);
    let mut player2_deck = Deck::from_input(&player2_input);

    play_game_of_combat(&mut player1_deck, &mut player2_deck);

    print!("Part 1 - ");
    if player1_deck.is_empty() {
        println!("Player 2 score: {}", player2_deck.calculate_score());
    } else {
        println!("Player 1 score: {}", player1_deck.calculate_score());
    }

    let mut recursive_combat = RecursiveCombatGame::from_inputs(
        "recursive combat", &player1_input, &player2_input
    );
    print!("Part 2 - ");
    match recursive_combat.play_till_winner() {
        Player::One => {
            println!("Player 1 score: {}", recursive_combat.player1_deck.calculate_score());
        }
        Player::Two => {
            println!("Player 2 score: {}", recursive_combat.player2_deck.calculate_score());
        }
    }

}
