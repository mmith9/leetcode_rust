struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut lastc = '-';
        let mut res = String::new();
        for c in s.chars() {
            if c != lastc {
                res.push(c); lastc = c
                }
            else {
                res.pop();
                if res.is_empty() {lastc = ' '}
                else {lastc = res.chars().last().unwrap()}
//                if let Some(somec) = res.chars().last() {lastc = somec} else {lastc = '-'}
            }
        }
        return res;
    }
}


pub fn main() {

    
}