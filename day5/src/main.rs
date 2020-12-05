use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

mod tests;

struct Range {
    start: u16,
    end: u16,
}

impl Range {
    fn new(start: u16, end: u16) -> Range {
        Range { start, end }
    }

    fn from_range_ref(range: &Range) -> Range {
        Range { ..*range }
    }

    fn keep_lower_halve(&self) -> Range {
        let mid: f32 = (self.start + self.end) as f32 / 2.0;
        Range { start: self.start, end: mid.floor() as u16 }
    }

    fn keep_upper_halve(&self) -> Range {
        let mid: f32 = (self.start + self.end) as f32 / 2.0;
        Range { start: mid.ceil() as u16, end: self.end }
    }
}

struct SeatsLayout {
    rows: Range,
    columns: Range
}

impl SeatsLayout {
    fn new() -> SeatsLayout {
        SeatsLayout {
            rows: Range::new(0, 127),
            columns: Range::new(0, 7)
        }
    }

    fn find_row(&self, partitioning_instructions: &Vec<char>) -> u16 {
        let mut range = Range::from_range_ref(&self.rows);

        for instruction in partitioning_instructions[..7].iter() {
            match instruction {
                'F' => { range = range.keep_lower_halve(); }
                'B' => { range = range.keep_upper_halve(); }
                _ => { panic!("Unknown instruction {}", instruction); }
            }
        }

        assert_eq!(range.start, range.end);

        range.start
    }

    fn find_column(&self, partitioning_instructions: &Vec<char>) -> u16 {
        let mut range = Range::from_range_ref(&self.columns);

        for instruction in partitioning_instructions[7..].iter() {
            match instruction {
                'L' => { range = range.keep_lower_halve(); }
                'R' => { range = range.keep_upper_halve(); }
                _ => { panic!("Unknown instruction {}", instruction); }
            }
        }

        assert_eq!(range.start, range.end);

        range.start
    }

    fn id_for_seat_at(&self, partitioning_instructions: String) -> u16 {
        let partitioning_instructions = partitioning_instructions.chars().collect();

        let row = self.find_row(&partitioning_instructions);
        let column = self.find_column(&partitioning_instructions);

        row * 8 + column
    }
}

fn get_input(filename: &str) -> Vec<String> {
    let file_path = Path::new(filename);
    let file = File::open(file_path);
    let file = file.unwrap();

    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut instructions: Vec<String> = vec![];
    for line in lines {
        let line = line.unwrap();
        if line.len() > 0 {
            instructions.push(String::from(line));
        }
    }

    return instructions;
}

fn main() {
    let instructions = get_input("input.txt");
    let seats_layout = SeatsLayout::new();

    let mut seat_ids = vec![];

    for instruction in instructions {
        let seat_id = seats_layout.id_for_seat_at(instruction);
        seat_ids.push(seat_id);
    }

    seat_ids.sort();
    for i in 0..seat_ids.len() - 1 {
        if seat_ids[i] + 1 != seat_ids[i + 1] {
            println!("{} is followed by {}", seat_ids[i], seat_ids[i+1]);
        }
    }
}
