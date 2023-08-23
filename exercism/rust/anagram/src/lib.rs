use std::collections::HashSet;
use counter::Counter;

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


fn letter_counts(word: &str) -> Counter<char> {
    word.chars().collect::<Counter<_>>()
}
