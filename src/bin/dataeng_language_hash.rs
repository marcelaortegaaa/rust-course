// Program that reads text from a file into a HashMap to count word frequencies in Rust.
use clap::Parser;
use std::collections::HashMap;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(long, help = "Path to file to count words of")]
    path: PathBuf,
}

fn word_counter(reader: BufReader<File>) -> io::Result<Vec<(String, u32)>> {
    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            let clean_word = word
                .trim_matches(|c: char| !c.is_alphanumeric())
                .to_lowercase();
            if !clean_word.is_empty() {
                *map.entry(clean_word).or_insert(0) += 1;
            }
        }
    }

    let mut result: Vec<(String, u32)> = map.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(result)
}

fn set_file<P: AsRef<Path>>(path: P) -> io::Result<BufReader<File>> {
    let file = OpenOptions::new().read(true).open(path)?;
    Ok(BufReader::new(file))
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let reader = set_file(args.path)?;
    let counted = word_counter(reader)?;

    println!("{counted:?}");
    Ok(())
}
