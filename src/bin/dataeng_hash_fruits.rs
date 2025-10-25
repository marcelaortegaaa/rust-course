use rand::{rng, seq::IndexedRandom};
use std::collections::HashSet;

fn generate_fruit(n: usize) -> HashSet<&'static str> {
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

fn main() {
    let n = 30;
    let fruit_set = generate_fruit(n);

    println!("Generating {} random fruits...", n);
    println!("Number of unique fruits generated: {}", fruit_set.len());
    println!("Fruit set: {:?}", fruit_set);
}
