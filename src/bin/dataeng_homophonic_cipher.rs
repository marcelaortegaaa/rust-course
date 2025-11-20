use rand::{rng, seq::IndexedRandom};
use std::collections::HashMap;

fn gen_key() -> HashMap<char, Vec<u8>> {
    let mut homophonic_key: HashMap<char, Vec<u8>> = HashMap::new();

    homophonic_key.insert('e', vec![01, 07, 15, 24, 33, 45, 56, 67, 78, 81, 90, 95]);
    homophonic_key.insert('t', vec![02, 11, 23, 35, 44, 58, 69, 77, 86, 92]);
    homophonic_key.insert('o', vec![03, 12, 28, 39, 48, 59, 70, 89]);
    homophonic_key.insert('i', vec![04, 17, 29, 38, 50, 61, 72, 83]);
    homophonic_key.insert('a', vec![05, 16, 27, 40, 51, 62, 74, 88]);
    homophonic_key.insert('n', vec![06, 19, 31, 47, 60, 71, 82]);
    homophonic_key.insert('r', vec![08, 20, 32, 46, 68, 84]);
    homophonic_key.insert('s', vec![09, 22, 37, 49, 73, 91]);
    homophonic_key.insert('h', vec![10, 25, 41, 57, 80]);
    homophonic_key.insert('l', vec![14, 34, 54, 75]);
    homophonic_key.insert('d', vec![18, 43, 64]);
    homophonic_key.insert('u', vec![21, 52, 79]);
    homophonic_key.insert('c', vec![26, 55, 85]);
    homophonic_key.insert('m', vec![30, 53, 87]);
    homophonic_key.insert('p', vec![36, 65]);
    homophonic_key.insert('y', vec![42, 66]);
    homophonic_key.insert('f', vec![63, 76]);
    homophonic_key.insert('g', vec![93, 97]);
    homophonic_key.insert('w', vec![94, 98]);
    homophonic_key.insert('b', vec![96, 99]);
    homophonic_key.insert('v', vec![00]);
    homophonic_key.insert('k', vec![13]);
    homophonic_key.insert('x', vec![45]);
    homophonic_key.insert('j', vec![56]);
    homophonic_key.insert('q', vec![67]);
    homophonic_key.insert('z', vec![78]);

    homophonic_key
}

fn encrypt(text: &str, key: HashMap<char, Vec<u8>>) -> String {
    let mut rng = rng();
    let mut result = String::new();

    for c in text.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            match key.get(&c) {
                Some(x) => {
                    let res = x.choose(&mut rng).cloned().unwrap();
                    result.push_str(&res.to_string());
                }
                None => result.push(c),
            }
        } else {
            result.push(c);
        }
    }

    result
}

fn main() {
    let key = gen_key();
    let message = "hello, what's up, i am a message";

    let encrypted_text = encrypt(message, key);
    println!("{}", encrypted_text);
}
