use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::game_of_waiting_area_seats::WaitingArea;

mod game_of_waiting_area_seats;
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
    let mut waiting_area = WaitingArea::new(&input);
    let mut ticks = 0;
    while waiting_area.tick2() {
        ticks += 1;
        if ticks % 100 == 0 {
            println!("Ticks: {}", ticks);
        }
    }
    println!("Occupied seats after stabilization: {}", waiting_area.get_number_of_occupied_seats());
}
