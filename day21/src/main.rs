use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::iter::FromIterator;
use std::path::Path;

use itertools::{all, join};
use regex::Regex;

mod tests;

#[derive(Debug, Clone, PartialEq)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl Food {
    pub fn from_description(input: &str) -> Food {
        let re = Regex::new(r"(.*) \(contains (.*)\)").unwrap();
        let groups = re.captures(input).unwrap();

        let ingredients = groups.get(1).unwrap().as_str();
        let allergens = groups.get(2).unwrap().as_str();

        let ingredients = ingredients.split_whitespace().map(|i| {
            String::from(i)
        }).collect();
        let allergens = allergens.split(", ").map(|a| {
            String::from(a)
        }).collect();

        Food { ingredients, allergens }
    }

    pub fn uses_ingredient(&self, ingredient: &str) -> bool {
        self.ingredients.contains(&String::from(ingredient))
    }
}

impl Display for Food {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{} ({})",
                 join(self.ingredients.iter(), ", "),
                 join(self.allergens.iter(), ", ")
        )
    }
}

#[derive(Debug)]
struct FoodList {
    foods: Vec<Food>
}

impl FoodList {
    pub fn from_input(input: Vec<String>) -> FoodList {
        let mut foods = vec![];

        for food_description in input.iter() {
            foods.push(
                Food::from_description(&food_description)
            );
        }

        FoodList { foods }
    }

    pub fn get_possible_allergens_for_ingredient(&self, ingredient: &str) -> Vec<String> {
        let ingredient = String::from(ingredient);
        let mut possible_allergens = HashSet::new();

        for food in self.foods.iter() {
            if food.ingredients.contains(&ingredient) {
                for allergen in food.allergens.iter() {
                    possible_allergens.insert(allergen.clone());
                }
            }
        }

        // Because I don't know how to convert a HashSet to a vector easily
        let mut possible_allergens_list = vec![];
        for possible_allergen in possible_allergens.iter() {
            possible_allergens_list.push(possible_allergen.clone());
        }

        possible_allergens_list.sort();
        possible_allergens_list
    }

    pub fn get_all_ingredients_with_possible_allergens(&self) -> Vec<(String, Vec<String>)> {
        let all_ingredients = self.get_all_ingredients();

        let mut ingredients_with_possible_allergens = vec![];

        for ingredient in all_ingredients.iter() {
            ingredients_with_possible_allergens.push(
                (ingredient.clone(), self.get_possible_allergens_for_ingredient(&ingredient))
            );
        }

        ingredients_with_possible_allergens.sort_by(|(i, _), (i_o, _)| {
            i.cmp(i_o)
        });

        ingredients_with_possible_allergens
    }

    pub fn is_valid_with_arrangement(&self, arrangement: &Arrangement) -> bool {
        for food in self.foods.iter() {
            let mut ingredients_in_this_food = HashSet::new();
            for ingredient in food.ingredients.iter() {
                ingredients_in_this_food.insert(ingredient.clone());
            }

            for allergen_in_this_food in food.allergens.iter() {
                let ingredient_for_allergen_in_current_arrangement =
                    arrangement.get_ingredient_for_allergen(&allergen_in_this_food);

                match ingredient_for_allergen_in_current_arrangement {
                    None => {
                        return false;
                    }
                    Some(ingredient) => {
                        if !ingredients_in_this_food.contains(&ingredient) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    pub fn find_valid_arrangement(&self) -> Option<Arrangement> {
        self._find_valid_arrangement(self.find_partial_arrangement(), self.get_all_ingredients())
    }

    pub fn get_possible_ingredients_for_allergen(&self, allergen: &str) -> Vec<String> {
        let mut current_ingredients_list: HashSet<String> = HashSet::new();
        let foods_with_this_allergen = self.get_foods_with_allergen(allergen);

        let first_food = foods_with_this_allergen.first().unwrap();
        for i in first_food.ingredients.iter() {
            current_ingredients_list.insert(i.clone());
        }

        for food in foods_with_this_allergen[1..].iter() {
            let mut this_food_ingredients_list = HashSet::new();
            for i in food.ingredients.iter() {
                this_food_ingredients_list.insert(i.clone());
            }

            current_ingredients_list = current_ingredients_list.intersection(
                &this_food_ingredients_list
            ).cloned().collect();
        }

        let mut ingredients = Vec::from_iter(current_ingredients_list);
        ingredients.sort();

        ingredients
    }

    pub fn find_partial_arrangement(&self) -> Arrangement {
        let mut allergen_to_possible_ingredients = HashMap::new();
        for allergen in self.get_all_allergens() {
            let mut possible_ingredients = self.get_possible_ingredients_for_allergen(&allergen);
            allergen_to_possible_ingredients.insert(
                allergen,
                possible_ingredients
            );
        }

        let mut arrangement = Arrangement::new();
        loop {
            let mut new_allergen_to_possible_ingredients = HashMap::new();
            let mut found_some_mapping = false;

            for (allergen, possible_ingredients) in allergen_to_possible_ingredients.iter() {
                if possible_ingredients.len() == 1 {
                    let found_ingredient = &possible_ingredients[0];

                    arrangement.add_allergen_for_ingredient(found_ingredient, allergen);

                    for (this_allergen, possible_ingredients) in allergen_to_possible_ingredients.iter() {
                        if allergen == this_allergen {
                            continue;
                        }

                        new_allergen_to_possible_ingredients.insert(
                            this_allergen.clone(),
                            possible_ingredients.iter().cloned().filter(|a| {
                                a != found_ingredient
                            }).collect()
                        );
                    }

                    found_some_mapping = true;
                    break;
                }

                new_allergen_to_possible_ingredients.insert(
                    allergen.clone(), possible_ingredients.clone()
                );
            }

            allergen_to_possible_ingredients = new_allergen_to_possible_ingredients;
            if !found_some_mapping {
                break;
            }
        }

        arrangement
    }

    pub fn solve_part1(&self, valid_arrangement: &Arrangement) -> usize {
        let used_ingredients: HashSet<String, RandomState> = HashSet::from_iter(valid_arrangement.get_all_ingredients());
        let available_ingredients = self.get_all_ingredients();

        let mut unused_ingredients = vec![];
        for i in available_ingredients {
            if !used_ingredients.contains(&i) {
                unused_ingredients.push(i);
            }
        }

        let mut number_of_times_unused_ingredients_appear = 0;
        for i in unused_ingredients.iter() {
            for food in self.foods.iter() {
                if food.uses_ingredient(&i) {
                    number_of_times_unused_ingredients_appear += 1;
                }
            }
        }

        number_of_times_unused_ingredients_appear
    }

    pub fn solve_part2(&self, valid_arrangement: &Arrangement) -> String {
        let mut ingredient_and_allergen_list: Vec<(String, String)> = valid_arrangement.ingredient_allergen.iter().map(|(i, a)| {
            (i.clone(), a.clone())
        }).collect();
        ingredient_and_allergen_list.sort_by_key(|(i, a)| {
            a.clone()
        });

        join(ingredient_and_allergen_list.iter().map(|(i, a)| {
            i.clone()
        }), ",")
    }

    fn get_foods_with_allergen(&self, allergen: &str) -> Vec<Food> {
        let mut foods = vec![];

        for food in self.foods.iter() {
            if food.allergens.contains(&String::from(allergen)) {
                foods.push(food.clone());
            }
        }

        foods
    }

    fn get_all_ingredients(&self) -> Vec<String> {
        let mut unique_ingredients = HashSet::new();
        for food in self.foods.iter() {
            for i in food.ingredients.iter() {
                unique_ingredients.insert(i.clone());
            }
        }

        Vec::from_iter(unique_ingredients)
    }

    fn get_all_allergens(&self) -> Vec<String> {
        let mut unique_allergens = HashSet::new();
        for food in self.foods.iter() {
            for i in food.allergens.iter() {
                unique_allergens.insert(i.clone());
            }
        }

        Vec::from_iter(unique_allergens)
    }

    fn _find_valid_arrangement(&self, current_arrangement: Arrangement, ingredients_to_arrange: Vec<String>) -> Option<Arrangement> {
        let all_allergens = self.get_all_allergens();

        if current_arrangement.contains_all_allergens(&all_allergens) {
            print!(".");
            std::io::stdout().flush();

            return if self.is_valid_with_arrangement(&current_arrangement) {
                Some(current_arrangement)
            } else {
                None
            };
        }

        if ingredients_to_arrange.is_empty() {
            return None;
        }

        let next_ingredient = ingredients_to_arrange[0].clone();
        let left_over_ingredients = ingredients_to_arrange[1..].to_vec();

        let possible_allergens = self.get_possible_allergens_for_ingredient(&next_ingredient);
        for allergen_to_try in possible_allergens {
            // Has this allergen already been assigned to an ingredient?
            // If so, we can't assign it to the current ingredient
            if current_arrangement.get_ingredient_for_allergen(&allergen_to_try).is_none() {
                // Try to find a valid arrangement when the current ingredient is assigned this allergen
                let mut new_arrangement = current_arrangement.clone();
                new_arrangement.add_allergen_for_ingredient(&next_ingredient, &allergen_to_try);

                match self._find_valid_arrangement(new_arrangement, left_over_ingredients.clone()) {
                    None => {}
                    Some(valid_arrangement) => {
                        return Some(valid_arrangement);
                    }
                }
            }

            // Try to find a valid arrangement without assigning the current ingredient any allergen
            let mut new_arrangement = current_arrangement.clone();
            match self._find_valid_arrangement(new_arrangement, left_over_ingredients.clone()) {
                None => {}
                Some(valid_arrangement) => {
                    return Some(valid_arrangement);
                }
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
struct Arrangement {
    ingredient_allergen: HashMap<String, String>
}

impl Arrangement {
    pub fn new() -> Arrangement {
        Arrangement { ingredient_allergen: HashMap::new() }
    }

    pub fn add_allergen_for_ingredient(&mut self, ingredient: &str, allergen: &str) {
        self.ingredient_allergen.insert(
            String::from(ingredient), String::from(allergen),
        );
    }

    pub fn get_ingredient_for_allergen(&self, allergen: &str) -> Option<String> {
        for (i, a) in self.ingredient_allergen.iter() {
            if a == allergen {
                return Some(i.clone());
            }
        }

        None
    }

    pub fn contains_all_allergens(&self, allergens_to_test: &Vec<String>) -> bool {
        let mut all_allergens = HashSet::new();
        for (_, allergen) in self.ingredient_allergen.iter() {
            all_allergens.insert(allergen.clone());
        }

        for allergen in allergens_to_test.iter() {
            if !all_allergens.contains(allergen) {
                return false;
            }
        }

        true
    }

    pub fn get_all_ingredients(&self) -> Vec<String> {
        let mut ingredients = vec![];

        for (i, _) in self.ingredient_allergen.iter() {
            ingredients.push(i.clone());
        }

        ingredients
    }
}

impl Display for Arrangement {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (ingredient, allergen) in self.ingredient_allergen.iter() {
            writeln!(f, "{} contains {}", ingredient, allergen);
        }

        Result::Ok(())
    }
}

impl Display for FoodList {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for food in self.foods.iter() {
            write!(f, "{}", food);
        }

        Result::Ok(())
    }
}

fn get_input(filename: &str) -> Vec<String> {
    let f = File::open(Path::new(filename)).unwrap();
    let lines = BufReader::new(f).lines();

    let mut input = vec![];

    for l in lines {
        input.push(l.unwrap());
    }

    input
}

fn main() {
    let input = get_input("input.txt");
    let food_list = FoodList::from_input(input);
    let arrangement = food_list.find_valid_arrangement();
    println!();
    if let Some(arrangement) = arrangement {
        println!("Part 1: {}", food_list.solve_part1(&arrangement));
        println!("Part 2: {}", food_list.solve_part2(&arrangement));
    } else {
        println!("No arrangement found.");
    }
}
