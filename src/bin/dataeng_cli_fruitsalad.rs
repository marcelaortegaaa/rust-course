// [X] Let user pick fruits
// [X] Print fruits in alphabetical order
// [X] Save fruits in CSV
// [X] Add random salad addition

use clap::Parser;
use library_project::cli_fruitsalad::{add_surprise, create_fruit_salad};
use std::{fs, path::PathBuf};

#[derive(Parser)]
#[command(version = "1.0", author = "Marcela ◡̈", about = "Fruits in a salad")]
struct Opts {
    #[arg(
        short,
        long,
        help = "Number of fruits in your salad",
        default_value_t = 5
    )]
    number: usize,
    #[arg(short, long, help = "Select fruits for salad")]
    pick: Option<Vec<String>>,
    #[clap(short, long, help = "Add fruits to list")]
    add: Option<String>,
    #[arg(short, long, help = "Add fruits through file")]
    csvfile: Option<String>,
    #[arg(long, help = "Add a surprise to the fruit salad")]
    add_surprise: bool,
    #[arg(long, help = "Path to save fruit-salad.csv")]
    output: Option<PathBuf>,
}

fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',').map(|s| s.trim().to_string()).collect()
}

fn display_fruit_salad(fruits: Vec<String>, surprise: String) {
    println!("Your fruit salad contains {} fruits:", fruits.len());

    for (i, item) in fruits.iter().rev().enumerate() {
        if i + 1 == fruits.len() {
            print!("{item}");
        } else {
            print!("{item}, ");
        }
    }
    println!("");

    if surprise.is_empty() {
        println!("No surprise this time");
    } else {
        println!("Your surprise addition is {surprise}! Yay!")
    }
}

fn save_fruit_salad(path: &mut PathBuf, fruit: &Vec<String>) {
    path.push("fruit_salad.csv");
    let contents = fruit.join(", ");
    if let Err(e) = fs::write(&path, contents) {
        eprintln!("Could not save fruit salad: {e}")
    };
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;
    let picked_fruits = match opts.pick {
        Some(_) => opts.pick.unwrap(),
        None => [].to_vec(),
    };

    let mut add_fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename).expect("Could not read file");
            csv_to_vec(&fruits)
        }
        None => opts
            .add
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
    };

    // Create the fruit salad
    let salad = create_fruit_salad(&mut add_fruit_list, num_fruits, picked_fruits);
    let surprise = add_surprise(opts.add_surprise);

    if opts.output.is_some() {
        save_fruit_salad(&mut opts.output.unwrap(), &salad);
    }

    display_fruit_salad(salad, surprise);
}
