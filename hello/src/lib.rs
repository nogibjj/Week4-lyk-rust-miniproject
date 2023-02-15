/* Count distinc anagrams*/

/* Accepts a string list.
Calculate the number anagrams in the file
*/
use std::collections::HashMap;

pub fn count_distinct_anagrams(s: String) -> usize {
    let strings = s.split_whitespace();
    let mut anagram_counts = HashMap::new();

    for string in strings {
        let mut chars: Vec<char> = string.chars().collect();
        chars.sort();
        let count = anagram_counts.entry(chars).or_insert(0);
        *count += 1;
    }

    anagram_counts.len()
}
