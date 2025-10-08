//! This library contains utils for normalizing inputs
//! The function available turns Strings into tuples that separate value and unit.
//! # Example
//! ```
//! use library_project::prepare_input;
//! let input = "3kg".to_string();
//! let normalized_input = prepare_input(&input);
//! ```

pub mod byte_unit;

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

/// Returns errors when turning inputs into bytes
#[derive(Debug)]
pub enum ParseSizeError {
    UnknownUnit(String),
    Overflow,
}

/// Turns value and unit into bytes
/// ```
/// use library_project::to_base_bytes;
/// to_base_bytes(45, "mb");
/// ```
pub fn to_base_bytes(number: u64, unit_str: &str) -> Result<u64, ParseSizeError> {
    let unit = byte_unit::ByteUnit::from_str(unit_str)
        .ok_or_else(|| ParseSizeError::UnknownUnit(unit_str.to_string()))?;

    number
        .checked_mul(unit.multiplier())
        .ok_or(ParseSizeError::Overflow)
}
