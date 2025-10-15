use clap::Parser;
use library_project::cli_fruitsalad::create_fruit_salad;

#[derive(Parser)]
#[command(
    version = "1.0",
    author = "Marcela ◡̈",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[arg(short, long)]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Create the fruit salad
    create_fruit_salad(num_fruits);

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits,
        create_fruit_salad(num_fruits)
    );
}
