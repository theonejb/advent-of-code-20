use std::collections::HashMap;

mod tests;

struct MemoryGame {
    // The number and the last 2 times it was spoken
    spoken_numbers: HashMap<u128, Vec<u128>>,
    last_spoken_number: u128,
    turn: u128,
    input: Vec<u128>,
}

impl MemoryGame {
    pub fn new(input: &[u128]) -> MemoryGame {
        MemoryGame {
            spoken_numbers: HashMap::new(),
            last_spoken_number: 0, // Doesn't count since we will update this with the first input number when it is spoken in the first turn
            turn: 0,
            input: Vec::from(input),
        }
    }

    pub fn next_number(&mut self) -> u128 {
        self.turn += 1;
        let turn = self.turn as usize;

        let next_number: u128;

        if turn <= self.input.len() {
            next_number = self.input[turn - 1];
        } else {
            let previous_turns_for_last_spoken_number = self.spoken_numbers.get(&self.last_spoken_number);
            match previous_turns_for_last_spoken_number {
                None => {
                    panic!("This can't happen");
                }
                Some(previous_turns) => {
                    if previous_turns.len() == 1 {
                        next_number = 0;
                    } else {
                        next_number = previous_turns[1] - previous_turns[0];
                    }
                }
            }
        }

        let next_number_spoken_turns = self.spoken_numbers.get_mut(&next_number);
        match next_number_spoken_turns {
            None => {
                self.spoken_numbers.insert(
                    next_number,
                    vec![self.turn]
                );
            }
            Some(turns) => {
                if turns.len() == 1 {
                    turns.push(self.turn)
                } else {
                    turns[0] = turns[1];
                    turns[1] = self.turn;
                }
            }
        }

        // println!("Turn: {}; Next Number: {}; Record: {:?}", turn, next_number, self.spoken_numbers.get(&next_number).unwrap());
        self.last_spoken_number = next_number;
        next_number
    }

    pub fn nth(&mut self, n: u128) -> u128 {
        loop {
            if self.turn == n - 1 {
                return self.next_number();
            } else {
                self.next_number();
            }
        }
    }
}

fn main() {
    let mut game = MemoryGame::new(&[11, 0, 1, 10, 5, 19]);
    println!("Part 1: {}", game.nth(2020));
    println!("Part 2: {}", game.nth(30_000_000));
}
