use std::collections::{HashMap, HashSet};

type CharChounter = HashMap<char, usize>;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_letter_counts = letter_counts(&word_lower);

    possible_anagrams
        .iter()
        .filter(|&possible_anagram| {
            let possible_anagram_lower = possible_anagram.to_lowercase();

            word_lower != possible_anagram_lower
                && word_letter_counts == letter_counts(&possible_anagram_lower)
        })
        .cloned()
        .collect()
}

fn letter_counts(word: &str) -> CharChounter {
    let mut counter = CharChounter::new();

    for c in word.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    counter
}
