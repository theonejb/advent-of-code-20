use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fmt;
use regex::Regex;

#[derive(Debug)]
pub struct Bag {
    name: String
}

impl Display for Bag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Bag {
    pub fn new(name: &str) -> Bag {
        Bag { name: String::from(name) }
    }
}

#[derive(Debug)]
pub struct Edge {
    to_bag_type_name: String,
    quantity: u16,
}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.to_bag_type_name, self.quantity)
    }
}

impl Edge {
    pub fn new(to_bag_name: &str, quantity: u16) -> Edge {
        Edge {
            to_bag_type_name: String::from(to_bag_name),
            quantity,
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    bags: Vec<Bag>,
    edges: HashMap<String, Vec<Edge>>,
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for bag in self.bags.iter() {
            let bag_name = &bag.name;
            write!(f, "{}\n\t", bag_name).unwrap();

            if self.edges.get(&bag.name).unwrap().is_empty() {
                writeln!(f, "*****").unwrap();
            } else {
                for edge in self.edges.get(&bag.name).unwrap().iter() {
                    write!(f, "{}/{} ", edge.to_bag_type_name, edge.quantity).unwrap();
                }

                writeln!(f).unwrap();
            }
        }

        fmt::Result::Ok(())
    }
}

enum DijkstraResult {
    Found(Vec<String>),
    NotFound
}

impl Graph {
    pub fn new() -> Graph {
        Graph { bags: vec![], edges: HashMap::new() }
    }

    pub fn add_rule_from_description(&mut self, rule_description: &str) {
        let re = Regex::new(r"^(.*?) bags contain (.*).$").unwrap();
        let cap = re.captures(rule_description);

        if let None = cap {
            return;
        }

        let cap = cap.unwrap();
        let from_bag_name = cap.get(1).unwrap().as_str();
        let to_bag_names = cap.get(2).unwrap().as_str();

        if to_bag_names == "no other bags" {
            self.add_empty_bag_rule(from_bag_name);
        } else {
            let re = Regex::new(r"(\d+)\s(.*?)\sbags?\b").unwrap();
            for cap in re.captures_iter(to_bag_names) {
                let to_bag_quantity = cap.get(1).unwrap().as_str().parse::<u16>().unwrap();
                let to_bag_name = cap.get(2).unwrap().as_str();

                self.add_rule(from_bag_name, to_bag_name, to_bag_quantity);
            }
        }
    }

    pub fn add_rule(&mut self, from_bag_name: &str, to_bag_name: &str, quantity: u16) {
        if !self.has_bag_type_with_name(from_bag_name) {
            self.bags.push(Bag::new(from_bag_name));
            self.edges.insert(from_bag_name.to_string(), vec![]);
        }

        if !self.has_bag_type_with_name(to_bag_name) {
            self.bags.push(Bag::new(to_bag_name));
            self.edges.insert(to_bag_name.to_string(), vec![]);
        }

        let from_bag_name = String::from(from_bag_name);
        let edge = Edge::new(&to_bag_name[..], quantity);

        self.edges.get_mut(&from_bag_name).unwrap().push(edge);
    }

    pub fn add_empty_bag_rule(&mut self, from_bag_name: &str) {
        if !self.has_bag_type_with_name(from_bag_name) {
            self.bags.push(Bag::new(from_bag_name));
        }

        self.edges.insert(from_bag_name.to_string(), vec![]);
    }

    pub fn length_of_rabbit_hole(&self, from_bag_name: &str) -> u32 {
        let edges = self.edges.get(from_bag_name).unwrap();
        let mut n = 0;

        for edge in edges.iter() {
            n += edge.quantity as u32;
            n += (edge.quantity as u32) * self.length_of_rabbit_hole(edge.to_bag_type_name.as_str());
        }

        n
    }

    pub fn find_route(&self, from_bag_name: &str, to_bag_name: &str) -> Option<Vec<String>> {
        match self.dijkstra(from_bag_name, to_bag_name) {
            DijkstraResult::Found(path) => Some(path),
            DijkstraResult::NotFound => None
        }
    }

    pub fn number_of_routes_to(&self, to_bag_name: &str) -> u32 {
        let mut n = 0;
        for bag in self.bags.iter() {
            if bag.name == to_bag_name {
                continue;
            }

            n += match self.find_route(bag.name.as_str(), to_bag_name) {
                Some(_) => 1,
                None => 0
            };
        }

        return n;
    }

    fn dijkstra(&self, from_bag_name: &str, to_bag_name: &str) -> DijkstraResult {
        if from_bag_name == to_bag_name {
            return DijkstraResult::Found(vec![]);
        }

        let edges_to_explore = self.edges.get(from_bag_name).unwrap();
        let mut bags_to_explore = vec![];

        for edge in edges_to_explore.iter() {
            bags_to_explore.push(edge.to_bag_type_name.as_str());
        }

        for current_bag in bags_to_explore.iter() {
            let current_bag = *current_bag;
            if let DijkstraResult::Found(path) = self.dijkstra(current_bag, to_bag_name) {
                let mut path = path;
                path.insert(0, String::from(from_bag_name));
                return DijkstraResult::Found(path);
            }
        }

        return DijkstraResult::NotFound
    }

    fn has_bag_type_with_name(&self, bag_name: &str) -> bool {
        for bag_type in self.bags.iter() {
            if bag_type.name == bag_name {
                return true;
            }
        }
        false
    }
}