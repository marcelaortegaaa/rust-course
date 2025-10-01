// Match Demo
// Didn't edit the course original too much
// Just added a branch when you say Hello
// And made it case insentive
// Learned about String and string literals as well, traumatizing process
// But rewarding

use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();
    let mut follow_up = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    // use of match expression to pattern match against variable "name"
    match name.to_lowercase().trim() {
        "good bye" => println!("Sorry to see you go."),
        "hello" => {
            println!("Hi, nice to meet you!");
            io::stdin()
                .read_line(&mut follow_up)
                .expect("Failed to read input");
            match follow_up.trim() {
                "how are you?" => println!("I'm doing well!"),
                _ => println!("Man, bye"),
            }
        }
        _ => println!("I can't find a greeting, good bye."),
    }
}
