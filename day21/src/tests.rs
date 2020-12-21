use crate::*;

fn get_input_lines() -> Vec<String> {
    vec![
        String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
        String::from("trh fvjkl sbzzf mxmxvkd (contains dairy)"),
        String::from("sqjhc fvjkl (contains soy)"),
        String::from("sqjhc mxmxvkd sbzzf (contains fish)"),
    ]
}

#[test]
fn test_food_from_description() {
    let food = Food::from_description(&get_input_lines()[0]);
    assert_eq!(food.ingredients, vec![
        String::from("mxmxvkd"),
        String::from("kfcds"),
        String::from("sqjhc"),
        String::from("nhms"),
    ]);
    assert_eq!(food.allergens, vec![String::from("dairy"), String::from("fish")]);

    let food = Food::from_description(&get_input_lines()[2]);
    assert_eq!(food.allergens, vec![String::from("soy")]);
}

#[test]
fn test_food_uses_ingredient() {
    let food = Food::from_description(&get_input_lines()[0]);
    assert!(food.uses_ingredient("kfcds"));
}

#[test]
fn test_food_list_get_possible_allergens_for_ingredient() {
    let mut food_list = FoodList::from_input(get_input_lines());
    assert_eq!(food_list.get_possible_allergens_for_ingredient("mxmxvkd"),
               vec![String::from("dairy"), String::from("fish")]);
}

#[test]
fn test_food_list_get_all_ingredients_with_possible_allergens() {
    let mut food_list = FoodList::from_input(get_input_lines());
    assert_eq!(food_list.get_all_ingredients_with_possible_allergens(), vec![
        (String::from("fvjkl"), vec![
            String::from("dairy"),
            String::from("soy"),
        ]),

        (String::from("kfcds"), vec![
            String::from("dairy"),
            String::from("fish"),
        ]),

        (String::from("mxmxvkd"), vec![
            String::from("dairy"),
            String::from("fish"),
        ]),

        (String::from("nhms"), vec![
            String::from("dairy"),
            String::from("fish"),
        ]),

        (String::from("sbzzf"), vec![
            String::from("dairy"),
            String::from("fish"),
        ]),

        (String::from("sqjhc"), vec![
            String::from("dairy"),
            String::from("fish"),
            String::from("soy"),
        ]),

        (String::from("trh"), vec![
            String::from("dairy"),
        ]),
    ]);
}

#[test]
fn test_food_list_is_valid_with_arrangement() {
    let mut food_list = FoodList::from_input(vec![
        String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
        String::from("sqjhc mxmxvkd sbzzf (contains fish)")
    ]);

    let mut arrangement = Arrangement::new();
    arrangement.add_allergen_for_ingredient("mxmxvkd", "soy");
    assert!(!food_list.is_valid_with_arrangement(&arrangement));

    let mut arrangement = Arrangement::new();
    arrangement.add_allergen_for_ingredient("mxmxvkd", "fish");
    arrangement.add_allergen_for_ingredient("sqjhc", "dairy");
    assert!(food_list.is_valid_with_arrangement(&arrangement));

    let mut arrangement = Arrangement::new();
    arrangement.add_allergen_for_ingredient("mxmxvkd", "fish");
    arrangement.add_allergen_for_ingredient("sqjhc", "soy");
    assert!(!food_list.is_valid_with_arrangement(&arrangement));
}

#[test]
fn test_arrangement_get_ingredient_for_allergen() {
    let mut arrangement = Arrangement::new();
    arrangement.add_allergen_for_ingredient("mxmxvkd", "fish");
    arrangement.add_allergen_for_ingredient("trh", "dairy");

    assert_eq!(arrangement.get_ingredient_for_allergen("dairy").unwrap(), String::from("trh"));
    assert_eq!(arrangement.get_ingredient_for_allergen("fish").unwrap(), String::from("mxmxvkd"));
    assert!(arrangement.get_ingredient_for_allergen("soy").is_none());
}

#[test]
fn test_food_list_find_valid_arrangement() {
    let food_list = FoodList::from_input(get_input_lines());
    match food_list.find_valid_arrangement() {
        None => {
            panic!("No valid arrangement found");
        }
        Some(arrangement) => {
            for ingredient in ["kfcds", "nhms", "sbzzf", "trh"].to_vec() {
                assert!(arrangement.get_ingredient_for_allergen(ingredient).is_none());
            }
        }
    }
}

#[test]
fn test_food_list_solve_part1() {
    let food_list = FoodList::from_input(get_input_lines());
    assert_eq!(food_list.solve_part1(), 5);
}

#[test]
fn test_food_list_find_partial_arrangement() {
    let food_list = FoodList::from_input(get_input_lines());
    println!("{}", food_list.find_partial_arrangement())
}