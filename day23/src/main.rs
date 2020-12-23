use std::collections::{VecDeque, HashSet};
use std::iter::FromIterator;
use itertools::{Itertools, join};

mod tests;

/*
The current_arrangement holds the cups in the order they are arranged in the game, with the current
cup in the back position. Keeping the cup in the back allows easily removing cups clockwise. It's
a pop_front() operation to remove the next clockwise cup.
 */
struct Game {
    current_arrangement: VecDeque<u32>,

    min_cup: u32,
    max_cup: u32,
}

impl Game {
    pub fn from_inputs(inputs: &Vec<u32>) -> Game {
        let mut current_arrangement = VecDeque::from_iter(inputs.iter().cloned());
        current_arrangement.rotate_left(1); // Bring the current cup to the back of the queue

        let sorted_cups: Vec<u32> = inputs.iter().cloned().sorted().collect();

        Game {
            current_arrangement,

            min_cup: *sorted_cups.first().unwrap(),
            max_cup: *sorted_cups.last().unwrap(),
        }
    }

    pub fn from_huge_inputs(inputs: &Vec<u32>, min: u32, max: u32) -> Game {
        let mut current_arrangement = VecDeque::from_iter(inputs.iter().cloned());
        current_arrangement.rotate_left(1); // Bring the current cup to the back of the queue

        Game {
            current_arrangement,

            min_cup: min,
            max_cup: max,
        }
    }

    pub fn pick_cups(&mut self, n: usize) -> Vec<u32> {
        let mut picked_cups = vec![];

        for _ in 0..n {
            picked_cups.push(self.current_arrangement.pop_front().unwrap());
        }

        picked_cups
    }

    pub fn get_designated_cup(&self, not_to_select: &Vec<u32>) -> u32 {
        let current_cup = *self.current_arrangement.back().unwrap();

        let mut possible_designated_cup = current_cup - 1;
        loop {
            if possible_designated_cup < self.min_cup {
                possible_designated_cup = self.max_cup;
            }

            if !not_to_select.contains(&possible_designated_cup) {
                return possible_designated_cup;
            }

            possible_designated_cup -= 1;
        }
    }

    fn get_current_cup(&self) -> u32 {
        *self.current_arrangement.back().unwrap()
    }

    fn make_cup_current(&mut self, cup: u32) {
        while self.get_current_cup() != cup {
            self.current_arrangement.rotate_right(1);
        }
    }

    pub fn place_cups(&mut self, designated_cup: u32, cups: &Vec<u32>) {
        let current_cup = self.get_current_cup();

        // Rotate the designated cup into the back, then add the cups to place at the front in reverse
        // order so they maintain their order in the clockwise direction
        self.make_cup_current(designated_cup);
        for cup in cups.iter().rev() {
            self.current_arrangement.push_front(*cup);
        }
        self.make_cup_current(current_cup);
    }

    pub fn select_next_current_cup(&mut self) {
        self.current_arrangement.rotate_left(1);
    }

    pub fn play_round(&mut self) {
        let picked_cups = self.pick_cups(3);
        let designated_cup = self.get_designated_cup(&picked_cups);

        self.place_cups(designated_cup, &picked_cups);
        self.select_next_current_cup();
    }

    pub fn print_current_arrangement(&self) {
        let current_cup = self.get_current_cup();
        print!("cups: ({}) ", current_cup);

        let mut cups_left_to_print = self.current_arrangement.len() - 1;
        for cup in self.current_arrangement.iter() {
            if cups_left_to_print == 0 {
                break;
            }
            cups_left_to_print -= 1;

            print!("{} ", cup);
        }

        println!();
    }
}

fn part1() {
    let inputs = vec![3, 6, 2, 9, 8, 1, 7, 5, 4];
    let mut game = Game::from_inputs(&inputs);

    println!("Starting part 1");
    for _ in 1..=1000 {
        game.print_current_arrangement();
        game.play_round();
    }

    game.make_cup_current(1);
    println!("-- final --");
    game.print_current_arrangement();
}

fn part2() {
    let mut inputs = vec![3, 6, 2, 9, 8, 1, 7, 5, 4];
    for cup in 10..=50 {
        inputs.push(cup);
    }
    let mut game = Game::from_huge_inputs(&inputs, 1, 50);
    println!("Starting part 2");
    for round in 1..=10000 {
        // if round % 10 == 0 {
        //     println!("-- move {} --", round);
        // }
        game.print_current_arrangement();
        game.play_round();
    }

    game.make_cup_current(1);
    println!("-- final --");
    println!("{}", join(game.pick_cups(5), ", "));
}

fn main() {
    // part1();
    part2();
}
