// Module 2: External Lab
// Create a File Reader Application
//
//To-Do:
// [ ] Let user specify the file path as a command-line argument
// [ ] Add error handling cases
// [ ] Read the file line-by-line and print it to the console

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("non_existent_file.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}

// Sample code to get an idea on how to get the first argument in the console.
// (Don't know what they mean by that)
//
//use std::env;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();

//     // The first argument is the path that was used to call the program.
//     println!("My path is {}.", args[0]);
// }
