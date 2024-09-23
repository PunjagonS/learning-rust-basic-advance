/*  -----------------------------------------------------
    Correct Search Results Using Word Grouping
        Description
        - Given a list if words, group the words that are anagrams

        Tools
        - HashMaps
        - Nested Loops
    -----------------------------------------------------
*/

use std::collections::HashMap;

/*
    The concept of this function can be understood as encoding each word into
    a frequency-based "hash" and then grouping words that share the same "hash."
    These words are anagrams of each other, meaning they have the same characters,
    the same character frequencies, but in different positions.
*/
fn word_groupings(words_list: Vec<String>) -> Vec<Vec<String>> {
    let mut word_hash = HashMap::new();

    // define vec! 26 lengths (0-25) with init 0 value ( vec![0,0,0,0, ... ] )
    let mut char_freq = vec![0; 26];

    for current_word in words_list {
        for c in current_word.to_lowercase().chars() {
            /*
                Calculate each char ascii to map position align with
                vec![26 index] define that `a` is index 0, `b` is index 1, ...
                and calculated index is incremented by 1 (frequency count).
            */
            char_freq[(c as u32 - 'a' as u32) as usize] += 1;
        }

        let key = char_freq
            .into_iter()
            .map(|i| i.to_string())
            .collect::<String>();
        word_hash
            .entry(key)
            .or_insert(Vec::new())
            .push(current_word);

        char_freq = vec![0; 26];
    }

    for (key, value) in &word_hash {
        println!("key # {:?} value # {:?}", key, value);
    }

    // old fasion to extract the value from hashmap by ignoring key `_`.
    // word_hash.into_iter().map(|(_, v)| v).collect()

    // using `into_values()` as compiler recommended to directly extract value only.
    word_hash.into_values().collect()
}

fn main() {
    let words = vec![
        "the".to_string(),
        "teh".to_string(),
        "het".to_string(),
        "stupid".to_string(),
        "studpi".to_string(),
        "stdupi".to_string(),
        "apple".to_string(),
        "appel".to_string(),
    ];

    let grouping = word_groupings(words);

    let input_word = String::from("stdupi");
    println!("Input word : {:?}", input_word);
    for i in grouping.into_iter() {
        if i.contains(&input_word) {
            println!("The group of word is {:?}", i);
        }
    }
}
