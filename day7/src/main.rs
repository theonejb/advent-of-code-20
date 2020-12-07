use std::path::Path;
use std::fs::{File};
use std::io::{BufReader, BufRead};
use crate::bags_graph::Graph;

mod bags_graph;
mod bags_graph_tests;

fn read_input_lines(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();

    let mut lines = vec![];
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        lines.push(line);
    }

    lines
}

fn main() {
    let input_lines = read_input_lines("input.txt");
    let mut graph = Graph::new();
    for line in input_lines.iter() {
        graph.add_rule_from_description(line.as_str());
    }
    println!("Number of ways to contain 'shiny gold': {}", graph.number_of_routes_to("shiny gold"));
    println!("Number of bags needed: {}", graph.length_of_rabbit_hole("shiny gold"));
}
