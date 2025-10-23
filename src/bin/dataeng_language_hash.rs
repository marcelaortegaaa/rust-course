// Program that reads text from a file into a HashMap to count word frequencies in Rust.

use std::collections::HashMap;

fn word_counter(words: Vec<&str>) -> Vec<(&str, u32)> {
    let mut frequencies = HashMap::new();

    for w in words {
        let frequency = frequencies.entry(w).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (w, frequency) in frequencies {
        result.push((w, frequency));
    }
    result.sort_by(|a, b| b.1.cmp(&a.1));

    result
}

fn main() {}
