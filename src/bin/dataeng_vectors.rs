// To Do:
// [X] Let user add fruits to the salad
// [X] Select a random fruit with .choose from rand
// [ ] Select a number of random fruits from a list and add to salad

use rand::rng;
use rand::seq::{IndexedRandom, SliceRandom};
use std::io;

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

    let mut extra_fruit = String::new();

    println!("Welcome to the Fruit Salad Bar");
    loop {
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

    // Pick a random fruit
    println!("Here's a random fruit: {}", fruit.choose(&mut rng).unwrap());

    // // Print out the fruit salad
    // println!("Fruit Salad:");
    // for (i, item) in fruit.iter().enumerate() {
    //     if i != fruit.len() - 1 {
    //         print!("{}, ", item);
    //     } else {
    //         println!("{}", item);
    //     }
    // }
}
