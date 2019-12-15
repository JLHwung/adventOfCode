use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::Read};

struct RecipeItem<'a> {
    name: &'a str,
    count: usize,
}

impl<'a> RecipeItem<'a> {
    pub fn parse(raw: &str) -> RecipeItem {
        let split: Vec<&str> = raw.split(" ").collect();
        RecipeItem {
            name: split[1],
            count: split[0].parse().unwrap(),
        }
    }
}

struct Recipe<'a> {
    serve: usize,
    ingredients: Vec<RecipeItem<'a>>,
}

type RecipeMap<'a> = HashMap<&'a str, Recipe<'a>>;
type ProductMap<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn find_rhs<'a>(
    recipes: &RecipeMap<'a>,
    products: &ProductMap<'a>,
    requests: &HashMap<&'a str, usize>,
) -> Option<&'a str> {
    for (&item, count) in requests.iter() {
        // resolved requests item is zero
        if *count == 0 {
            continue;
        }
        if !recipes.contains_key(item) {
            continue;
        }
        match products.get(item) {
            Some(products) => {
                if products.iter().all(|&product| {
                    recipes.contains_key(product)
                        && requests.contains_key(product)
                        && *requests.get(product).unwrap() == 0usize
                }) {
                    return Some(item);
                }
            }
            None => {
                return Some(item);
            }
        }
    }
    None
}

fn satisfy<'a>(
    recipes: &RecipeMap<'a>,
    products: &ProductMap<'a>,
    requests: &mut HashMap<&'a str, usize>,
) {
    loop {
        match find_rhs(recipes, products, requests) {
            Some(request_item) => match recipes.get(request_item) {
                Some(recipe) => {
                    let count = requests.get(request_item).unwrap();
                    let mut min_quantity = count / recipe.serve;
                    if count % recipe.serve != 0 {
                        min_quantity += 1;
                    }
                    recipe.ingredients.iter().for_each(|x| {
                        let booked = requests.entry(x.name).or_insert(0);
                        *booked += x.count * min_quantity
                    });
                    let resolved = requests.get_mut(request_item).unwrap();
                    *resolved = 0usize;
                }
                None => {}
            },
            None => break,
        }
    }
}

fn parse_recipes(raw: &String) -> RecipeMap {
    raw.split("\n").fold(HashMap::new(), |mut acc, line: &str| {
        let caps: Vec<&str> = line.split(", ").collect();
        let dest = caps[caps.len() - 1];
        let dest_recipe = RecipeItem::parse(dest);
        acc.insert(
            dest_recipe.name,
            Recipe {
                serve: dest_recipe.count,
                ingredients: caps[0..caps.len() - 1]
                    .iter()
                    .map(|&x| RecipeItem::parse(x))
                    .collect(),
            },
        );
        acc
    })
}

fn generate_derivatives<'a>(recipes: &RecipeMap<'a>) -> ProductMap<'a> {
    let mut product_map = HashMap::new();
    for (&item, recipe) in recipes {
        recipe.ingredients.iter().for_each(|recipe_item| {
            let acc_item: &mut HashSet<&str> = product_map
                .entry(recipe_item.name)
                .or_insert(HashSet::new());
            acc_item.insert(item);
        });
    }
    product_map
}

fn main() -> io::Result<()> {
    let mut file = File::open("./data/input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content = content.trim_end().replace(" => ", ", ");

    let recipes = parse_recipes(&content);

    let product_map = generate_derivatives(&recipes);

    let ore_capacity: usize = 1000000000000;

    let mut fuel_target_min = ore_capacity / 1046814;
    let mut fuel_target_max = fuel_target_min * 3;

    while fuel_target_max - fuel_target_min >= 2 {
        let fuel_target = (fuel_target_min + fuel_target_max) / 2;

        let mut requests = HashMap::new();

        requests.insert("FUEL", fuel_target);

        satisfy(&recipes, &product_map, &mut requests);

        let ore_consumed = requests.get("ORE").unwrap();

        if *ore_consumed < ore_capacity {
            fuel_target_min = fuel_target
        } else {
            fuel_target_max = fuel_target
        }
    }

    println!("{}", (fuel_target_min + fuel_target_max) / 2);
    Ok(())
}
