mod test;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Bus {
    id: u32,
    offset: u32
}

impl Bus {
    pub fn time_since_last_departure(&self, from_timestamp: u32) -> u32 {
        from_timestamp % self.id
    }

    pub fn time_till_next_departure(&self, starting_from_timestamp: u32) -> u32 {
        self.id - self.time_since_last_departure(starting_from_timestamp)
    }
}

struct Input {
    earliest_arrival_timestamp: u32,
    busses: Vec<Option<Bus>>,
}

impl Input {
    pub fn get_valid_busses(&self) -> Vec<&Bus> {
        let mut valid_busses = vec![];
        for bus in self.busses.iter() {
            if let Some(bus) = bus {
                valid_busses.push(bus);
            }
        }

        valid_busses
    }

    pub fn get_earliest_available_bus_and_wait_time(&self) -> (&Bus, u32) {
        let busses = self.get_valid_busses();

        let mut wait_time = busses[0].time_till_next_departure(self.earliest_arrival_timestamp);
        let mut earliest_available_bus = busses[0];

        for bus in &busses[1..] {
            let this_bus_wait_time = bus.time_till_next_departure(self.earliest_arrival_timestamp);
            if this_bus_wait_time < wait_time {
                wait_time = this_bus_wait_time;
                earliest_available_bus = bus;
            }
        }

        (earliest_available_bus, wait_time)
    }

    pub fn solve2(&self) {
        println!("Solve using these equations at: https://www.dcode.fr/chinese-remainder");

        let busses = self.get_valid_busses();
        for bus in busses {
            let remainder = bus.id as i32 - (bus.offset as i32 % bus.id as i32);
            println!("x = {} mod {}", remainder.abs(), bus.id);
        }
    }
}

fn notes_to_busses(note: &str) -> Vec<Option<Bus>> {
    let input_busses = note.split(',');

    let mut busses = vec![];
    for (offset, bus_id) in input_busses.enumerate() {
        if bus_id == "x" {
            busses.push(None);
        } else {
            let bus_id = bus_id.parse::<u32>().unwrap();
            busses.push(Some(Bus { id: bus_id, offset: offset as u32 }));
        }
    }

    busses
}
fn get_input(filename: &str) -> Input {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let mut lines = BufReader::new(file).lines();

    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();

    let earliest_arrival_timestamp = first_line.parse::<u32>().unwrap();
    let busses = notes_to_busses(&second_line[..]);

    Input { earliest_arrival_timestamp, busses }
}

fn main() {
    let input = get_input("input.txt");
    let (bus, wait_time) = input.get_earliest_available_bus_and_wait_time();
    println!("Bus id: {}; Wait time: {}", bus.id, wait_time);
    println!("Result part 1: {}", bus.id * wait_time);
    input.solve2();
}
