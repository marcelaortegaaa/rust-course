// [X] Include ability to remove a fruit from the set
// [X] Print fruits in reverse
// [X] Track how many times each fruit is generated with repetition

use clap::Parser;
use rand::{rng, seq::IndexedRandom, seq::SliceRandom};
use std::collections::{BTreeMap, BTreeSet};

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

fn create_repeated_fruit_set<'a>(n: usize, fruits: &'a Vec<&'a str>) -> Vec<&'a str> {
    let mut rng = rng(); // There may be a better option than defining rng twice

    let fruit_set = (0..n)
        .map(|_| fruits.choose(&mut rng).copied().unwrap()) // with replacement
        .collect();

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
    let repeated_fruit_set = create_repeated_fruit_set(n, &fruits);

    if let Some(removed) = removed_fruit.as_ref() {
        for fruit in removed {
            if fruit_set.remove(fruit.to_lowercase().as_str()) {
                println!("Removed {} from set", fruit.to_lowercase());
                println!("New set of {}: {:?}", fruit_set.len(), fruit_set);
            }
        }
    }

    println!("\nReverse order:");

    for (i, item) in fruit_set.iter().rev().enumerate() {
        if i + 1 == fruit_set.len() {
            print!("{item}");
        } else {
            print!("{item}, ");
        }
    }
    println!("");

    // Count how many times each fruit is generated with repetition
    let mut frequencies = BTreeMap::new();
    for f in &repeated_fruit_set {
        let frequency = frequencies.entry(f).or_insert(0);
        *frequency += 1;
    }
    println!("Repeated fruit vec: {repeated_fruit_set:?}");
    println!("Repeated fruit sequences: {frequencies:?}");
}
