use std::collections::VecDeque;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn get_test_input() -> Vec<i64> {
    [
        35,
        20,
        15,
        25,
        47,
        40,
        62,
        55,
        65,
        95,
        102,
        117,
        150,
        182,
        127,
        219,
        299,
        277,
        309,
        576
    ].to_vec()
}

fn get_input(filename: &str) -> Vec<i64> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut input_numbers = vec![];
    for line in lines {
        let line = line.unwrap();
        let number = line.parse::<i64>().unwrap();
        input_numbers.push(number);
    }

    input_numbers
}

fn is_valid_number(component_numbers: &mut VecDeque<i64>, number: i64) -> bool {
    let component_numbers = component_numbers.make_contiguous();
    for (i, n) in component_numbers[..component_numbers.len() - 1].iter().enumerate() {
        for m in component_numbers[i..].iter() {
            if m + n == number {
                return true;
            }
        }
    }

    false
}

fn find_weakness(input: &Vec<i64>, invalid_number: i64) -> Vec<i64> {
    // We go to input.len - 2 because we know for a fact that no 2 numbers in the input add up
    // to the invalid number, so we need at least 3 contiguous numbers
    for i in 0..input.len() - 2 {
        let mut number_of_inputs_to_consider = 3usize;
        loop {
            let sum: i64 = input[i..i+number_of_inputs_to_consider].iter().sum();
            if sum == invalid_number {
                return input[i..i+number_of_inputs_to_consider].to_vec();
            } else if sum > invalid_number {
                break
            } else {
                number_of_inputs_to_consider += 1;
            }
        }
    }

    return vec![];
}

fn find_invalid_number(input: &Vec<i64>, preamble_length: usize) -> i64 {
    let mut buffer: VecDeque<i64> = VecDeque::new();

    for input_element in input.iter() {
        if buffer.len() < preamble_length {
            buffer.push_back(*input_element);
        } else {
            if !is_valid_number(&mut buffer, *input_element) {
                return *input_element;
            } else {
                buffer.pop_front();
                buffer.push_back(*input_element);
            }
        }
    }

    0
}

fn main() {
    let input = get_input("input.txt");

    let invalid_number = find_invalid_number(&input, 25);
    println!("Invalid number: {}", invalid_number);

    let mut encryption_weakness = find_weakness(&input, invalid_number);
    encryption_weakness.sort();
    println!("Encryption weakness: {}", encryption_weakness.first().unwrap() + encryption_weakness.last().unwrap());
}
