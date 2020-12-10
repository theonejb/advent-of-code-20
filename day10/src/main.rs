use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn get_input(filename: &str) -> Vec<i32> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut input: Vec<i32> = vec![];
    for line in lines {
        let line = line.unwrap();
        let input_element = line.parse().unwrap();
        input.push(input_element);
    }

    input
}

fn get_histogram_of_joltage_jumps(adapter_chain: &Vec<i32>) -> HashMap<i32, i32> {
    let mut histogram = HashMap::new();

    // The difference b/w our starting joltage (0J) and our first adapter
    histogram.insert(*adapter_chain.last().unwrap(), 1);
    // The difference b/w our device and last adapter is always 3
    histogram.insert(3, 1);


    for (i, adapter) in adapter_chain[..adapter_chain.len()-1].iter().enumerate() {
        let joltage_jump = adapter - adapter_chain[i+1];
        let count = match histogram.get(&joltage_jump) {
            None => 0,
            Some(c) => c + 1
        };
        histogram.insert(joltage_jump, count);
    }

    histogram
}

fn main() {
    let mut input = get_input("input.txt");
    input.sort();
    input.reverse();

    let histogram = get_histogram_of_joltage_jumps(&input);
    let final_answer = histogram.get(&1).unwrap() * histogram.get(&3).unwrap();
    println!("{}", final_answer);
}
