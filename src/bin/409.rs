struct Solution;


use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut freqs:HashMap<char,i32> = HashMap::new();
        for c in s.chars() {

            //freqs.entry(c).and_modify(|freq| *freq +=1).or_insert(1);

            *freqs.entry(c).or_insert(0) += 1;

            // if let Some(freq) = freqs.get(&c) {
            //     freqs.insert(c, freq+1);
            // } else { freqs.insert(c,1);}
        }
        let mut res = 0;
        let mut odd = false;
        for val in freqs.values() {
            if val & 1 == 1 {odd = true; res += val -1}
            else {res += val}
        }
        if odd {res+=1}

        return res;
    }
}



pub fn main() {

    let s = "abccccdd".to_string();
    let result = Solution::longest_palindrome(s);
    println!("{result} 7");

}