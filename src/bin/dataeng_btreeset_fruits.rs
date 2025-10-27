// [ ] Include ability to remove a fruit from the set
// [ ] Print fruits in reverse
// [ ] Track how many times each fruit is generated with repetition

use clap::Parser;
use rand::rng;
use rand::seq::SliceRandom;
use std::collections::BTreeSet;

#[derive(Parser)]
struct Args {
    #[arg(short, long, help = "Remove fruit from set")]
    remove: Option<Vec<String>>,
    #[arg(short, long, help = "Number of fruits for set", default_value_t = 5)]
    number: usize,
}

fn create_fruit_set<'a>(n: usize, fruits: &'a Vec<&'a str>) -> BTreeSet<&'a str> {
    let mut fruit_set = BTreeSet::new();

    for fruit in fruits {
        fruit_set.insert(fruit.to_owned());
        if fruit_set.len() >= n {
            break;
        }
    }

    println!("{}: {:?}", n, fruit_set);
    fruit_set
}

fn main() {
    let args = Args::parse();
    let removed_fruit = args.remove;
    let n = args.number;

    let mut fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
    ];

    let mut rng = rng();
    fruits.shuffle(&mut rng);

    let mut fruit_set = create_fruit_set(n, &fruits);

    if let Some(removed) = removed_fruit.as_ref() {
        for fruit in removed {
            if fruit_set.remove(fruit.to_lowercase().as_str()) {
                println!("Removed {} from set", fruit.to_lowercase());
                println!("New set of {}: {:?}", fruit_set.len(), fruit_set);
            }
        }
    }
}
