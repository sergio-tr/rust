use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    fn sort_word(word: &str) -> Vec<char> {
        let mut chars: Vec<char> = word.to_uppercase().as_str().chars().collect();
        chars.sort_unstable();
        chars
    }

    let sorted_word = sort_word(word);

    let mut anagrams = HashSet::new();

    for &possible_anagram in possible_anagrams {

        if  possible_anagram.to_lowercase() == word.to_lowercase() {
            continue;
        }

        if sorted_word == sort_word(possible_anagram){
            anagrams.insert(possible_anagram);
        }
    }

    anagrams
}
