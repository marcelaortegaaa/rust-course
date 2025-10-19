// [X] Let user pass input
// [ ] Count words in a sentence
// [ ] Sort result by frequency

use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[command(
    version = "1.0",
    author = "Marcela ◡̈",
    about = "Counter for numbers in a vector"
)]
struct Opts {
    #[arg(
        long,
        value_delimiter = ',',
        num_args = 1..,
        value_parser = clap::value_parser!(i32),
        default_values_t = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3],
        help = "Vector to count the numbers of"
    )]
    vector: Vec<i32>,
}

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    let opts: Opts = Opts::parse();
    let user_input = opts.vector;

    let result = logic(user_input);

    //print the results in a human readable format that explains what the result is.
    println!(
        "The frequency of each number in the vector is: {:?}",
        result
    );
}
