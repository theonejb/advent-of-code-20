use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

fn get_input() -> Vec<i32> {
    let input_path = Path::new("input.txt");
    let f = match File::open(input_path) {
        Err(why) => panic!("could not open {}: {}", input_path.display(), why),
        Ok(file) => file
    };

    let lines = BufReader::new(f).lines();

    let mut input_numbers: Vec<i32> = vec![];
    for l in lines.into_iter() {
        let line = l.unwrap();
        let number = line.parse::<i32>().unwrap();
        input_numbers.push(number);
    }

    return input_numbers;
}

fn main() {
    let mut input = get_input();
    input.sort();

    let input_el_count = input.len();

    for (i, v1) in input[..input_el_count - 2].iter().enumerate() {
        for (j, v2) in input[i + 1..input_el_count - 1].iter().enumerate() {
            if v1 + v2 > 2020 {
                continue;
            }

            for v3 in input[j + 1..].iter() {
                if v1 + v2 + v3 == 2020 {
                    println!("{} * {} * {} = {}", v1, v2, v3, v1 * v2 * v3);
                    return;
                }
            }
        }
    }

    println!("Not found");
}
