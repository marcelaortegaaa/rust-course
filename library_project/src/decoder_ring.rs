use std::{char, collections::HashMap, f32};

fn gen_counts() -> HashMap<char, f32> {
    // Reference letter frequencies in English
    let mut eng_freq: HashMap<char, f32> = HashMap::new();

    // Accounts for 80% of all letters in English
    eng_freq.insert('e', 12.7);
    eng_freq.insert('t', 9.1);
    eng_freq.insert('a', 8.2);
    eng_freq.insert('o', 7.5);
    eng_freq.insert('i', 7.0);
    eng_freq.insert('n', 6.7);
    eng_freq.insert('s', 6.3);
    eng_freq.insert('h', 6.1);
    eng_freq.insert('r', 6.0);
    eng_freq.insert('d', 4.3);

    // The rest of the letters
    eng_freq.insert('l', 4.0);
    eng_freq.insert('c', 2.8);
    eng_freq.insert('u', 2.8);
    eng_freq.insert('m', 2.4);
    eng_freq.insert('w', 2.4);
    eng_freq.insert('f', 2.2);
    eng_freq.insert('g', 2.0);
    eng_freq.insert('y', 2.0);
    eng_freq.insert('p', 1.9);
    eng_freq.insert('b', 1.5);
    eng_freq.insert('v', 0.98);
    eng_freq.insert('k', 0.77);
    eng_freq.insert('j', 0.16);
    eng_freq.insert('x', 0.15);
    eng_freq.insert('q', 0.12);
    eng_freq.insert('z', 0.074);

    eng_freq
}

fn stats_analysis(text: &str) -> Vec<(char, u32, f32, Option<f32>, f32)> {
    let mut counts: HashMap<char, u32> = HashMap::new();

    for c in text.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let total: u32 = counts.values().sum();

    let eng_freq_map = gen_counts();
    let eng_freq_map: HashMap<char, f32> = eng_freq_map.iter().map(|(k, v)| (*k, *v)).collect();

    let mut results = Vec::new();

    for (letter, count) in &counts {
        let freq = (*count as f32 / total as f32) * 100.0;
        let eng_freq = eng_freq_map.get(&letter.to_ascii_lowercase()).cloned();

        let eng_freq_diff = eng_freq.map_or(0.0, |f| (freq - f).abs());

        results.push((*letter, *count, freq, eng_freq, eng_freq_diff));
    }
    results
}

pub fn print_stats_analysis(text: &str) {
    let stats = stats_analysis(text);
    for (letter, count, freq, eng_freq, eng_freq_diff) in stats {
        println!(
            "{}: {} ({:.3}%), English Freq: {} ({:.3}%)",
            letter,
            count,
            freq,
            eng_freq.unwrap_or(0.0),
            eng_freq_diff
        );
    }
}

pub fn decrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base + shift) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }

    result
}

// Find the most frequent letter and shift assuming it's one of the top 3 most frequent letters in English (e, t, or o)
// I could have hardcoded the most frequent letters in English but added the extra code in case the letters in gen_counts() were to change. Like if they changed to Spanish letter frequencies maybe.
pub fn alternate_guess(text: &str) {
    let stats = stats_analysis(text);

    let mut text_freqs = Vec::new();
    let mut eng_freqs = Vec::new();

    for (char, _, freq, eng_freq, _) in stats {
        if char.is_alphabetic() {
            text_freqs.push((char, freq));
            if char.is_lowercase() {
                eng_freqs.push((char, eng_freq.unwrap()));
            }
        }
    }

    text_freqs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    eng_freqs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let text_letter = text_freqs[0].0 as u8;
    let eng_letter: Vec<_> = eng_freqs
        .into_iter()
        .take(3)
        .map(|(ch, _)| ch as u8)
        .collect();

    let shift_guess: Vec<_> = eng_letter
        .into_iter()
        .map(|l| ((l as i16 - text_letter as i16 + 26) % 26) as u8)
        .collect();

    println!("SHIFT GUESSES: {:?}", shift_guess);

    for shift in shift_guess {
        let decrypted_text = decrypt(text, shift);
        let stats = stats_analysis(&decrypted_text);

        let mut score = 0.0;
        for (_, _, freq, eng_freq, eng_freq_diff) in stats {
            if let Some(eng_freq) = eng_freq {
                score += (1.0 - eng_freq_diff / eng_freq) * freq;
            }
        }
        println!("Shift: {}, {}, Score: {}", shift, decrypted_text, score);
    }
}

/*
Guess Shift:

First, uses statistical analysis to determine the most likely shift.
Then, uses the most likely shift to decrypt the message.
Accepts:
 * text: the message to decrypt
 * depth: the number of shifts to try
Returns:
   * depth: the number of shifts tried
   * shift: the most likely shift
   * decrypted: the decrypted message
   * max score: highest score
*/

pub fn guess_shift(text: &str, depth: u8) -> (u8, u8, String, f32) {
    let mut max_score = f32::NEG_INFINITY; // For lower scores, but at that point guessing gets unreliable
    let mut best_shift = 0;
    let mut decrypted = String::new();

    for shift in 0..depth {
        let decrypted_text = decrypt(text, shift);
        let stats = stats_analysis(&decrypted_text);

        let mut score = 0.0;
        for (_, _, freq, eng_freq, eng_freq_diff) in stats {
            if let Some(eng_freq) = eng_freq {
                score += (1.0 - eng_freq_diff / eng_freq) * freq;
            }
        }
        println!("Shift: {}, {}, Score: {}", shift, decrypted_text, score);
        if score > max_score {
            max_score = score;
            best_shift = shift;
            decrypted = decrypted_text;
        }
    }

    (depth, best_shift, decrypted, max_score)
}
