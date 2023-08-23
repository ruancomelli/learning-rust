use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&possible_anagram| is_anagram(word, possible_anagram))
        .cloned()
        .collect()
}

fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    unimplemented!(
        "Determine if '{possible_anagram}' is an anagram of '{word}'",
        word = word,
        possible_anagram = possible_anagram
    );
}
