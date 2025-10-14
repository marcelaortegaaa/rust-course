// To Do:
// [X] Let user add fruits to the salad
// [X] Select a random fruit with .choose from rand
// [X] Select a number of random fruits from a list and add to salad

use rand::rng;
use rand::seq::{IndexedRandom, SliceRandom};
use std::io::{self, Write};

fn read_salad_size() -> usize {
    loop {
        io::stdout().flush().ok();
        println!("How many fruits would you like in your fruit salad?");
        let mut s = String::new();
        if io::stdin().read_line(&mut s).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }
        match s.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => eprintln!("Please type a non-negative whole number."),
        }
    }
}

fn main() {
    let mut fruit = vec![
        "Orange".to_string(),
        "Fig".to_string(),
        "Pomegranate".to_string(),
        "Cherry".to_string(),
        "Apple".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
    ];

    println!("Welcome to the Fruit Salad Bar");
    loop {
        let mut extra_fruit = String::new();
        extra_fruit.clear();
        println!("\nHere are the fruits we currently have");
        for (i, fr) in fruit.iter().enumerate() {
            if i != fruit.len() - 1 {
                print!("{}, ", fr);
            } else {
                println!("{}", fr)
            }
        }
        println!("Would you like to add another fruit?");

        io::stdin()
            .read_line(&mut extra_fruit)
            .expect("Didn't catch that. Please repeat your fruit.");

        let trimmed_fruit = extra_fruit.trim();

        if trimmed_fruit.eq_ignore_ascii_case("no") {
            break;
        };
        fruit.push(trimmed_fruit.to_string());
    }

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    fruit.shuffle(&mut rng);

    let fruit_number = read_salad_size();

    let fruit_salad: Vec<String> = fruit
        .choose_multiple(&mut rng, fruit_number)
        .cloned()
        .collect();

    // Print out the fruit salad
    println!("Here's your fruit salad:");
    for (i, item) in fruit_salad.iter().enumerate() {
        if i != fruit_salad.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
