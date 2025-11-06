// [X] Let user pick fruits
// [X] Print fruits in alphabetical order

use clap::Parser;
use library_project::cli_fruitsalad::create_fruit_salad;

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
}

fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',').map(|s| s.trim().to_string()).collect()
}

fn display_fruit_salad(fruits: Vec<String>) {
    println!("Your fruit salad contains {} fruits:", fruits.len());

    for (i, item) in fruits.iter().rev().enumerate() {
        if i + 1 == fruits.len() {
            print!("{item}");
        } else {
            print!("{item}, ");
        }
    }
    println!("");
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

    display_fruit_salad(salad);
}
