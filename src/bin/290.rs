struct Solution;


use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut letters = pattern.chars();
        let mut dict:HashMap<char, String> = HashMap::new();
        let mut mapped_words:HashSet<String> = HashSet::new();

        for word in s.split(' ') {
            if let Some(c) = letters.next() {
                if let Some(word2) = dict.get(&c) {
                    if word2 != word {return false}
                } else {
                    if mapped_words.contains(word) {return false}
                    dict.insert(c, word.to_string());
                    mapped_words.insert(word.to_string());
                }
            } else {return false}
        }
        if let Some(_not_exhausted) = letters.next() {return false}
        return true;
    }
}

// can/should do
// let words:Vec<&str>=s.split(' ').collect();
// if words.len() != pattern.len() {return false}
//
// then can do: 
// let c = letters.next().unwrap()
// omit _not_exhaused unpack


pub fn main() {



}