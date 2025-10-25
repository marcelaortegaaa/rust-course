use rand::{rng, seq::IndexedRandom};
use std::collections::HashSet;

fn generate_fruit(n: usize) -> HashSet<&'static str> {
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

    let mut rng = rng();

    let fruit_set = fruits.choose_multiple(&mut rng, n).cloned().collect();
    fruit_set
}

fn main() {
    let n = 100;
    let fruit_set = generate_fruit(n);

    println!("Generating {} random fruits...", n);
    println!("Number of unique fruits generated: {}", fruit_set.len());
}
