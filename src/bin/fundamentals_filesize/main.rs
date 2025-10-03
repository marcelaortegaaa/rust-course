// Module 3: External Lab
// Create a File Size Formatter
//
//To-Do:
// [ ] Get String with size and unit and return Struct with representations in KB, MB, etc.
// [ ] Split the input string to capture number and the sizes

use std::env;

fn prepare_input(s: String) -> (String, String) {
    let reduced = s.to_lowercase().split_whitespace().collect::<String>();

    match reduced.find(|c: char| c.is_alphabetic()) {
        Some(i) => {
            let (left, right) = reduced.split_at(i);
            (left.to_string(), right.to_string())
        }
        None => (reduced.to_string(), "".to_string()),
    }
}

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn main() {
    let arg = env::args().nth(1); // Can be merged on the match below
    println!("argument: {:?}", arg);

    let file_size = match arg {
        Some(input) => prepare_input(input),
        None => {
            println!("You have not provided a file size.");
            return;
        }
    };

    println!("{:?}", file_size)

    // let result = format_size(6888837399);
    // println!("{}", result)
}
