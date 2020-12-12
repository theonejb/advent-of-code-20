use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::ferry_simulator::{Ship, MovementInstruction};

mod ferry_simulator;
mod tests;

fn get_input(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut input = vec![];
    for line in lines {
        input.push(line.unwrap());
    }

    input
}

fn main() {
    let input = get_input("input.txt");
    //59816
    let mut ship = Ship::new();
    for instruction in input.iter() {
        let movement_instruction = MovementInstruction::compile(&instruction[..]);
        ship.follow_instruction(&movement_instruction);
    }

    println!("Manhattan distance: {}", ship.get_manhattan_distance());
}
