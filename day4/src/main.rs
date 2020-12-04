use std::io::{BufReader, BufRead, Lines};
use std::fs::{File};
use std::path::Path;
use std::fmt::{Display, Formatter};

fn get_input_iterator(filename: &str) -> Lines<BufReader<File>> {
    let file_path = Path::new(filename);
    let file = File::open(file_path).expect("Unable to open file.");

    BufReader::new(file).lines()
}

struct PassportEntry {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: String,
}

impl Display for PassportEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "byr:{} iyr:{} eyr:{} hgt:{} hcl:{} ecl:{} pid:{} cid:{}",
               self.birth_year, self.issue_year, self.expiration_year, self.height, self.hair_color,
               self.eye_color, self.passport_id, self.country_id)
    }
}

impl PassportEntry {
    fn is_empty(&self) -> bool {
        self.birth_year.len() == 0 &&
            self.issue_year.len() == 0 &&
            self.expiration_year.len() == 0 &&
            self.height.len() == 0 &&
            self.hair_color.len() == 0 &&
            self.eye_color.len() == 0 &&
            self.passport_id.len() == 0 &&
            self.country_id.len() == 0
    }

    fn is_birth_year_valid(&self) -> bool {
        match self.birth_year.parse::<i32>() {
            Ok(year) => (year >= 1920) && (year <= 2002),
            Err(_) => false
        }
    }

    fn is_issue_year_valid(&self) -> bool {
        match self.issue_year.parse::<i32>() {
            Ok(year) => (year >= 2010) && (year <= 2020),
            Err(_) => false
        }
    }

    fn is_expiration_year_valid(&self) -> bool {
        match self.expiration_year.parse::<i32>() {
            Ok(year) => (year >= 2020) && (year <= 2030),
            Err(_) => false
        }
    }

    fn is_height_valid(&self) -> bool {
        if self.height.len() < 3 {
            return false;
        }

        let unit = &self.height[self.height.len()-2..];
        let measurement_str = &self.height[..self.height.len()-2];
        let measurement: i32;
        match measurement_str.parse() {
            Ok(x) => {
                measurement = x
            }
            Err(_) => {
                return false;
            }
        }

        if unit == "cm" {
            return (measurement >= 150) && (measurement <= 193);
        } else if unit == "in" {
            return (measurement >= 59) && (measurement <= 76);
        }

        false
    }

    fn is_hair_color_valid(&self) -> bool {
        if self.hair_color.len() != 7 {
            return false;
        }

        if self.hair_color.chars().nth(0).unwrap() != '#' {
            return false;
        }

        for c in self.hair_color[1..].chars() {
            if !c.is_numeric() && !matches!(c, 'a'..='f') {
                return false;
            }
        }

        return true;
    }

    fn is_eye_color_valid(&self) -> bool {
        [
            "amb".to_string(),
            "blu".to_string(),
            "brn".to_string(),
            "gry".to_string(),
            "grn".to_string(),
            "hzl".to_string(),
            "oth".to_string()
        ].contains(
            &self.eye_color
        )
    }

    fn is_passport_id_valid(&self) -> bool {
        if self.passport_id.len() != 9 {
            return false;
        }

        match self.passport_id.parse::<i64>() {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn is_country_id_valid(&self) -> bool {
        true
    }

    fn is_valid(&self) -> bool {
        self.is_birth_year_valid() &&
            self.is_issue_year_valid() &&
            self.is_expiration_year_valid() &&
            self.is_height_valid() &&
            self.is_hair_color_valid() &&
            self.is_eye_color_valid() &&
            self.is_passport_id_valid() &&
            self.is_country_id_valid()
    }

    fn update_with_input_key_value(&mut self, key: &str, value: &str) {
        if key == "byr" {
            self.birth_year = String::from(value);
        } else if key == "iyr" {
            self.issue_year = String::from(value);
        } else if key == "eyr" {
            self.expiration_year = String::from(value);
        } else if key == "hgt" {
            self.height = String::from(value);
        } else if key == "hcl" {
            self.hair_color = String::from(value);
        } else if key == "ecl" {
            self.eye_color = String::from(value);
        } else if key == "pid" {
            self.passport_id = String::from(value);
        } else if key == "cid" {
            self.country_id = String::from(value);
        }
    }
}

fn read_entry(lines_iterator: &mut Lines<BufReader<File>>) -> Option<PassportEntry> {
    let mut passport_entry = PassportEntry {
        birth_year: "".to_string(),
        issue_year: "".to_string(),
        expiration_year: "".to_string(),
        height: "".to_string(),
        hair_color: "".to_string(),
        eye_color: "".to_string(),
        passport_id: "".to_string(),
        country_id: "".to_string(),
    };

    for line in lines_iterator {
        let line = line.unwrap();
        if line.trim().len() == 0 {
            break;
        }

        let words = line.split_whitespace();
        for key_pair in words {
            let key_pair_parts: Vec<&str> = key_pair.split(':').collect();

            if key_pair_parts.len() != 2 {
                continue;
            }

            let key = key_pair_parts[0];
            let value = key_pair_parts[1];

            passport_entry.update_with_input_key_value(key, value);
        }
    }

    if passport_entry.is_empty() {
        None
    } else {
        Some(passport_entry)
    }
}

fn read_all_entries(input_iterator: &mut Lines<BufReader<File>>) -> Vec<PassportEntry> {
    let mut entires = vec![];

    loop {
        let next_entry = read_entry(input_iterator);
        match next_entry {
            None => { break; }
            Some(passport_entry) => { entires.push(passport_entry) }
        }
    }

    entires
}

fn main() {
    let mut input_iterator = get_input_iterator("input.txt");
    let passport_entries = read_all_entries(&mut input_iterator);

    let mut valid_entries = 0usize;
    for passport_entry in passport_entries {
        if passport_entry.is_valid() {
            valid_entries += 1;
        }
    }

    println!("Number of valid entries: {}", valid_entries);
}
