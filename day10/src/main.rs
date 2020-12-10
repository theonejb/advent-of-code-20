mod tests;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::AdapterTestResults::{JoltsShallNotPass, FoundAWay};
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

#[derive(Debug)]
enum AdapterTestResults {
    FoundAWay(Vec<i32>),
    JoltsShallNotPass
}

fn test_all_adapters(current_joltage: i32, remaining_adapters: &Vec<i32>) -> AdapterTestResults {
    if remaining_adapters.is_empty() {
        return FoundAWay(vec![]);
    }

    for (i, adapter) in remaining_adapters.iter().enumerate() {
        let joltage_difference = adapter - current_joltage;

        if joltage_difference >= 1 && joltage_difference <= 3 {
            let mut other_adapters = remaining_adapters.clone();
            other_adapters.remove(i);

            if let FoundAWay(adapter_chain) = test_all_adapters(
                *adapter, &other_adapters
            ) {
                let mut chain_with_this_adapter = adapter_chain;
                chain_with_this_adapter.push(*adapter);
                return FoundAWay(chain_with_this_adapter);
            }
        }
    }

    JoltsShallNotPass
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
    let chain = test_all_adapters(0, &input);
    if let FoundAWay(chain) = chain {
        let histogram = get_histogram_of_joltage_jumps(&chain);
        let final_answer = histogram.get(&1).unwrap() * histogram.get(&3).unwrap();
        println!("{}", final_answer);
    } else {
        println!("Unable to find a way.");
    }
}
