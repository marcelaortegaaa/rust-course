fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    // println!("{}", &sentence[2..=4]);

    // concatenate using format!
    // let description = format!("Title: Quick story\n{}", sentence);
    // println!("{}", sentence);

    // The 'Function Stole my String' Special
    // let description = sentence + "\nTitle: Quick story";

    // Limited to the end
    // sentence.push_str("\nTitle: Quick story");

    // -----------------

    // iterate over the characters in the sentence
    // let mut count_a = 0;
    // let mut count_e = 0;
    // let mut count_i = 0;
    // let mut count_o = 0;
    // let mut count_u = 0;

    // for c in sentence.chars() {
    //     match c {
    //         'a' => count_a += 1,
    //         'e' => count_e += 1,
    //         'i' => count_i += 1,
    //         'o' => count_o += 1,
    //         'u' => count_u += 1,
    //         _ => continue,
    //     }
    // }

    // println!(
    //     "a: {}, e: {}, i: {}, o: {}, u: {}",
    //     count_a, count_e, count_i, count_o, count_u
    // );

    // There may be a more efficient way to do this, I can't think of it. Maybe Vectors?

    // -----------------

    // Split and collect into a vector
    // let words = sentence.split(' ').collect::<Vec<&str>>();

    fn longest_word(sentence: &String) {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        let max_len = words.iter().map(|s| s.len()).max().unwrap_or(0);
        let longest: Vec<&str> = words
            .iter()
            .filter(|s| s.len() == max_len)
            .copied()
            .collect();

        println!(
            "The longest word(s) is/are: {:?}, with {} characters",
            longest, max_len
        );
    }

    longest_word(&sentence);

    let test = "hey, is this thing working?".to_string();
    longest_word(&test);

    // let reversed = sentence.chars().rev().collect::<String>();
    // println!("{}", reversed);
}
