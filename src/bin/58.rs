struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        return s.trim().split(' ').last().unwrap().len() as i32;
    }
}


pub fn main() {
    let s = "some words to test  ".to_string();
    let result = Solution::length_of_last_word(s);
    println!("{}", result)

}