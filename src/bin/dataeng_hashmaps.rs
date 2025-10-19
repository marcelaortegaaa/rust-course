// [X] Let user pass input
// [X] Count words in a sentence
// [X] Sort result by frequency

use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[command(
    version = "1.0",
    author = "Marcela ◡̈",
    about = "Counter for numbers in a vector",
    name = "counter"
)]
#[group(id = "mode", required = true, multiple = false)]
struct Opts {
    #[arg(
        long,
        value_delimiter = ',',
        num_args = 1..,
        value_parser = clap::value_parser!(i32),
        help = "Vector to count the numbers of"
    )]
    vector: Option<Vec<i32>>,

    #[arg(long, help = "String to count the words of")]
    sentence: Option<String>,
}

fn number_logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }
    result.sort_by(|a, b| b.1.cmp(&a.1));

    result
}

fn sentence_logic(words: Vec<&str>) -> Vec<(&str, u32)> {
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

fn main() {
    let opts: Opts = Opts::parse();

    match (opts.vector, opts.sentence) {
        (Some(v), None) => {
            let result = number_logic(v);
            println!(
                "The frequency of each number in the vector is: {:?}",
                result
            );
        }
        (None, Some(s)) => {
            let words: Vec<&str> = s.split_whitespace().collect();
            let result = sentence_logic(words);
            println!(
                "The frequency of each word in the sentence is: {:?}",
                result
            );
        }
        _ => unreachable!("Pass either a vector or a sentence"),
    }
}
