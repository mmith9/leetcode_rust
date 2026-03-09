
struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let bans: HashSet<String> = HashSet::from_iter(banned.iter().map(|x| x.to_lowercase()));
        let mut text = paragraph;
        let space = " ".to_string();
        println!("{text}");
        for c in "!?',;.".chars() {
            text = text.replace(c, &space);
            println!("{text}");
        }
        let mut freqs:HashMap<String, i32> = HashMap::new();
        for word in text.split_whitespace() {
            if word.len() == 0 {continue}
            let word = word.to_string().to_lowercase();
            if bans.contains(&word) {continue}
            freqs.entry(word).and_modify(|x| *x+=1).or_insert(0);
        }

        //let maxfreq = *freqs.values().max().unwrap();
        //freqs.into_iter().filter_map(|(word, freq)| (freq == maxfreq).then_some(word)).next().unwrap()
        freqs.into_iter().max_by_key(|(_, freq)| *freq).map(|(word, _)| word).unwrap()
    }
}

pub fn main() {


}