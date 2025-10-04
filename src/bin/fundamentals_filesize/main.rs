// Module 3: External Lab
// Create a File Size Formatter
//
//To-Do:
// [ ] Get String with size and unit and return Struct with representations in KB, MB, etc.
// [X] Split the input string to capture number and the sizes
// [ ] Create the struct and the function for struct instances

use std::env;
use std::num::ParseIntError;

fn prepare_input(s: String) -> (u64, String) {
    let reduced = s.to_lowercase().split_whitespace().collect::<String>();

    match reduced.find(|c: char| c.is_alphabetic()) {
        Some(i) => {
            let (left, right) = reduced.split_at(i);
            (left.parse().unwrap(), right.to_string())
        }
        None => (reduced.parse().unwrap(), "".to_string()),
    }
}

#[derive(Debug, Clone, Copy)]
enum Unit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl Unit {
    /// Map user text → Unit. Add all synonyms you want to accept here.
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "b" | "byte" | "bytes" => Some(Unit::Bytes),
            "kb" | "kbs" | "kilobyte" | "kilobytes" => Some(Unit::Kilobytes),
            "mb" | "megabyte" | "megabytes" => Some(Unit::Megabytes),
            "gb" | "gigabyte" | "gigabytes" => Some(Unit::Gigabytes),
            "" => Some(Unit::Bytes),
            _ => None,
        }
    }

    fn multiplier(self) -> u64 {
        match self {
            Unit::Bytes => 1,
            Unit::Kilobytes => 1_000,
            Unit::Megabytes => 1_000_000,
            Unit::Gigabytes => 1_000_000_000,
        }
    }
}

#[derive(Debug)]
enum ParseSizeError {
    UnknownUnit(String),
    Overflow,
}

fn to_base_bytes(number: u64, unit_str: &str) -> Result<u64, ParseSizeError> {
    let unit = Unit::from_str(unit_str)
        .ok_or_else(|| ParseSizeError::UnknownUnit(unit_str.to_string()))?;

    number
        .checked_mul(unit.multiplier())
        .ok_or(ParseSizeError::Overflow)
}

// fn format_size(size: u64) -> String {
//     let filesize = match size {
//         0..=999 => FileSize::Bytes(size),
//         1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
//         1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
//         _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
//     };

//     match filesize {
//         FileSize::Bytes(bytes) => format!("{} bytes", bytes),
//         FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
//         FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
//         FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
//     }
// }

#[derive(Debug)]
struct Sizes {
    bytes: u64,
    kilobytes: u64,
    megabytes: u64,
    gigabytes: u64,
}

impl Sizes {
    fn new(base_bytes: u64) -> Self {
        let kb = base_bytes / 1_000;
        let mb = base_bytes / 1_000_000;
        let gb = base_bytes / 1_000_000_000;

        Self {
            bytes: base_bytes,
            kilobytes: kb,
            megabytes: mb,
            gigabytes: gb,
        }
    }
}

fn main() {
    let arg = env::args().nth(1); // Can be merged on the match below
    println!("argument: {:?}", arg);

    let prepared_input = match arg {
        Some(input) => prepare_input(input),
        None => {
            println!("You have not provided a file size.");
            return;
        }
    };
    println!("{:?}", prepared_input);

    let base_bytes = to_base_bytes(prepared_input.0, prepared_input.1.as_str());
    println!("base bytes: {:?}", base_bytes);

    // match (&file_size.0, file_size.1.as_str()) {
    //     (a, "kb" | "kbs" | "kilobytes") => {
    //         let base_bytes = a * 1000;
    //         println!("In: {}", base_bytes);
    //         let size_struct = Sizes::new(base_bytes);
    //         println!("{:#?}", size_struct)
    //     }
    //     _ => println!("Out of bounds"),
    // }

    // let result = format_size(6888837399);
    // println!("{}", result)
}
