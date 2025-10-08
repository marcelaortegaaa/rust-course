//! This library contains utils for normalizing inputs
//! The function available turns Strings into tuples that separate value and unit.
//! # Example
//! ```
//! use library_project::prepare_input;
//! let input = "3kg".to_string();
//! let normalized_input = prepare_input(&input);
//! ```

/// This function takes a String and normalizes it into a tuple with an unsigned integer and a lowercase String.
/// It's meant to be used on strings that contain a value and a unit.
/// It doesn't matter if your value and unit are separate or together
/// # Example
/// ```
/// use library_project::prepare_input;
/// let input = "3kg".to_string();
/// let normalized_input = prepare_input(&input);
/// ```
pub fn prepare_input(s: &String) -> (u64, String) {
    let reduced = s.to_lowercase().split_whitespace().collect::<String>();

    match reduced.find(|c: char| c.is_alphabetic()) {
        Some(i) => {
            let (left, right) = reduced.split_at(i);
            (left.parse().unwrap(), right.to_string())
        }
        None => (reduced.parse().unwrap(), "".to_string()),
    }
}

/// For normalized inputs for units of digital information (*bytes*)
#[derive(Debug, Clone, Copy)]
pub enum ByteUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl ByteUnit {
    ///Accepts multiple instances and classifies into variants
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "b" | "byte" | "bytes" => Some(ByteUnit::Bytes),
            "kb" | "kbs" | "kilobyte" | "kilobytes" => Some(ByteUnit::Kilobytes),
            "mb" | "megabyte" | "megabytes" => Some(ByteUnit::Megabytes),
            "gb" | "gigabyte" | "gigabytes" => Some(ByteUnit::Gigabytes),
            "" => Some(ByteUnit::Bytes),
            _ => None,
        }
    }

    ///Conversion rate from any unit into bytes
    pub fn multiplier(self) -> u64 {
        match self {
            ByteUnit::Bytes => 1,
            ByteUnit::Kilobytes => 1_000,
            ByteUnit::Megabytes => 1_000_000,
            ByteUnit::Gigabytes => 1_000_000_000,
        }
    }

    ///Labels for displaying
    pub fn label(self) -> &'static str {
        match self {
            ByteUnit::Bytes => "bytes",
            ByteUnit::Kilobytes => "kilobytes",
            ByteUnit::Megabytes => "megabytes",
            ByteUnit::Gigabytes => "gigabytes",
        }
    }
}
