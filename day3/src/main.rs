use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

type Grid = Vec<Vec<char>>;

#[derive(Debug)]
struct Map {
    grid: Grid,
    pattern_width: usize,
    height: usize,
}

fn get_input(filename: &str) -> Map {
    let input_path = Path::new(filename);
    let f = File::open(input_path).expect("Unable to open file");
    let lines = BufReader::new(f).lines();

    let mut grid: Grid = vec![];
    for line in lines {
        let line = line.unwrap();
        let mut row = vec![];

        for terrain in line.chars() {
            row.push(terrain);
        }

        grid.push(row);
    }

    Map {
        pattern_width: grid[0].len(),
        height: grid.len(),
        grid,
    }
}

fn run_you_fools(map: &Map, speed: (usize, usize)) -> i32 {
    let mut x = 0usize;
    let mut y = 0usize;
    let mut number_of_trees_encountered = 0;

    loop {
        if map.grid[y][x] == '#' {
            number_of_trees_encountered += 1;
        }

        if y == map.height - 1 {
            break;
        }

        x = (x + speed.0) % map.pattern_width;
        y += speed.1;
    }

    number_of_trees_encountered
}

fn main() {
    let map = get_input("input.txt");
    let mut mult_result = 1u64;

    for speed in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let number_of_trees = run_you_fools(&map, *speed) as u64;
        println!("Number of trees encountered at speed {:?} =  {}", speed, number_of_trees);

        mult_result *= number_of_trees;
    }

    println!("Multiplication result: {}", mult_result);
}
