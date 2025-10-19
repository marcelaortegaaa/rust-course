// [X] Let user pick fruits
// [X] Print fruits in alphabetical order

use clap::Parser;
use library_project::cli_fruitsalad::create_fruit_salad;

#[derive(Parser)]
#[command(
    version = "1.0",
    author = "Marcela ◡̈",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[arg(
        short,
        long,
        help = "Number of fruits in your salad",
        default_value_t = 10
    )]
    number: usize,

    #[arg(short, long, help = "Select fruits for salad")]
    pick: Option<Vec<String>>,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;
    let picked_fruits = match opts.pick {
        Some(_) => opts.pick.unwrap(),
        None => [].to_vec(),
    };

    // Create the fruit salad
    let salad = create_fruit_salad(num_fruits, picked_fruits);

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        salad.len(),
        salad
    );
}
