use std::env;
use std::fmt;

pub fn prepare_input(s: String) -> (u64, String) {
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

pub fn to_base_bytes(number: u64, unit_str: &str) -> Result<u64, ParseSizeError> {
    let unit = Unit::from_str(unit_str)
        .ok_or_else(|| ParseSizeError::UnknownUnit(unit_str.to_string()))?;

    number
        .checked_mul(unit.multiplier())
        .ok_or(ParseSizeError::Overflow)
}
