// Module 2: External Lab
// Create a File Reader Application
//
//To-Do:
// [X] Let user specify the file path as a command-line argument
// [X] Add error handling cases
// [X] Read the file line-by-line and print it to the console
// [X] Write on the files
//
// I get what they mean by command line argument now
// I should also make it so users can write what they want to append
// But I can't be bothered to do it right now
// It's called file reader anyways

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Your path is {:?}", args[1]);

    let file = File::open(&args[1]);
    let file = match file {
        Ok(_) => match OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(&args[1])
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open or create file: {}", e);
                return;
            }
        },
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                println!("File not found: {}", error);
                println!("Do you want to create the file? (y/n)");

                let mut choice = String::new();
                std::io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read input");
                let choice = choice.trim().to_lowercase();

                if choice == "y" {
                    match File::create(&args[1]) {
                        Ok(_) => {
                            println!("Your file was created");
                            match OpenOptions::new()
                                .read(true)
                                .write(true)
                                .append(true)
                                .open(&args[1])
                            {
                                Ok(file) => file,
                                Err(e) => {
                                    eprintln!("Failed to reopen created file: {}", e);
                                    return;
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to create file: {}", e);
                            return;
                        }
                    }
                } else {
                    println!("File was not created.");
                    return;
                }
            }
            std::io::ErrorKind::PermissionDenied => {
                eprintln!("You do not have access to this file: {}", error);
                return;
            }
            _ => {
                eprintln!("Error opening file: {}", error);
                return;
            }
        },
    };

    let reader = BufReader::new(&file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => eprintln!("Error reading line: {}", error),
        }
    }

    mark_file(&file);
}

fn mark_file(mut file: &File) {
    println!("Hey, now that we're here. Do you wanna stamp the file as opened? (y/n)");

    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice = choice.trim().to_lowercase();

    if choice == "y" {
        writeln!(file, "\nThis file has been opened by The File Reader")
            .expect("Failed to add notice to file");
        println!("Per your request, message printed.")
    } else {
        println!("Per your request, message not printed.")
    }
}
