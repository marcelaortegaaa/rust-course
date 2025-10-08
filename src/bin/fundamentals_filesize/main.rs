// Module 3: External Lab
// Create a File Size Formatter
//
//To-Do:
// [X] Get String with size and unit and return Struct with representations in KB, MB, etc.
// [X] Split the input string to capture number and the sizes
// [X] Create the struct and the function for struct instances
// [ ] Make numbers floats

use library_project::prepare_input;
use std::env;
use std::fmt;

// fn prepare_input(s: String) -> (u64, String) {
//     let reduced = s.to_lowercase().split_whitespace().collect::<String>();

//     match reduced.find(|c: char| c.is_alphabetic()) {
//         Some(i) => {
//             let (left, right) = reduced.split_at(i);
//             (left.parse().unwrap(), right.to_string())
//         }
//         None => (reduced.parse().unwrap(), "".to_string()),
//     }
// }

#[derive(Debug, Clone, Copy)]
enum Unit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl Unit {
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

    fn label(self) -> &'static str {
        match self {
            Unit::Bytes => "bytes",
            Unit::Kilobytes => "kilobytes",
            Unit::Megabytes => "megabytes",
            Unit::Gigabytes => "gigabytes",
        }
    }
}

#[derive(Debug)]
enum ParseSizeError {
    UnknownUnit(String),
    Overflow,
}

fn to_base_bytes(number: u64, unit_str: &str) -> Result<u64, ParseSizeError> {
    let unit = ByteUnit::from_str(unit_str)
        .ok_or_else(|| ParseSizeError::UnknownUnit(unit_str.to_string()))?;

    number
        .checked_mul(unit.multiplier())
        .ok_or(ParseSizeError::Overflow)
}

fn with_unit(n: u64, u: Unit) -> String {
    format!("{} {}", n, u.label())
}

struct Sizes {
    bytes: u64,
    kilobytes: u64,
    megabytes: u64,
    gigabytes: u64,
}

impl fmt::Debug for Sizes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sizes")
            .field("bytes", &with_unit(self.bytes, Unit::Bytes))
            .field("kilobytes", &with_unit(self.kilobytes, Unit::Kilobytes))
            .field("megabytes", &with_unit(self.megabytes, Unit::Megabytes))
            .field("gigabytes", &with_unit(self.gigabytes, Unit::Gigabytes))
            .finish()
    }
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
    let prepared_input = match env::args().nth(1) {
        Some(input) => prepare_input(input),
        None => {
            println!("You have not provided a file size.");
            return;
        }
    };

    let base_bytes = match to_base_bytes(prepared_input.0, prepared_input.1.as_str()) {
        Ok(b) => b,
        Err(ParseSizeError::UnknownUnit(u)) => {
            eprintln!("Unknown unit: {u}");
            return;
        }
        Err(ParseSizeError::Overflow) => {
            eprintln!("Size too large (overflow)");
            return;
        }
    };

    let size_struct = Sizes::new(base_bytes);
    println!("{:#?}", size_struct)
}
