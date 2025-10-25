// Generating Unique Fruits with HashSet
// [X] Generate a user-specified number of random fruits
// [X] Track how many times each fruit is generated with repetition
// [X] Print set of fruits

use clap::Parser;
use rand::{rng, seq::IndexedRandom};
use std::collections::HashSet;

#[derive(Parser)]
struct Args {
    #[arg(short, long, help = "Number of fruits for set", default_value_t = 10)]
    number: usize,
}

fn generate_fruits(n: usize) -> HashSet<&'static str> {
    let mut rng = rng();
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];

    let mut fruit_set = HashSet::new();
    for _ in 0..n {
        let fruit = fruits.choose(&mut rng).cloned().unwrap();
        fruit_set.insert(fruit);
    }

    // This makes the HashSet unnecesary because the method doesn't have repetition
    // let fruit_set = fruits.choose_multiple(&mut rng, n).cloned().collect();

    fruit_set
}

// fn generate_repeated_fruits(n: usize) -> VecDeque<&'static str> {
//     let mut rng = rng();
//     let fruits = [
//         "Apple",
//         "Banana",
//         "Cherry",
//         "Date",
//         "Elderberry",
//         "Fig",
//         "Grape",
//         "Honeydew",
//     ];

//     let mut fruit_set = VecDeque::new();
//     for _ in 0..n {
//         let fruit = fruits.choose(&mut rng).cloned().unwrap();
//         fruit_set.push_back(fruit);
//     }

//     fruit_set
// }

fn main() {
    let args: Args = Args::parse();
    let n = args.number;
    let fruit_set = generate_fruits(n);

    // // Count how many times each fruit is generated with repetition
    // let mut frequencies = HashMap::new();
    // for f in &fruit_set {
    //     let frequency = frequencies.entry(f).or_insert(0);
    //     *frequency += 1;
    // }
    // println!("{frequencies:?}");

    println!("Generating {} random fruits", n);
    println!("Number of fruits generated: {}", fruit_set.len());
    println!("Fruit set: {:?}", fruit_set);
}
