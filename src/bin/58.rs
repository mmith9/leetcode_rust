struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut skip = 0;
        let mut res = 0;

        for a in s.chars().rev() {
            if a == ' ' {skip +=1}
            else {break}
        }

        for a in s.chars().rev().skip(skip) {
            if a != ' ' {res +=1}
            else {break}
        }

        return res
    }

    pub fn length_of_last_word2(s: String) -> i32 {
        return s.trim().split(' ').last().unwrap().len() as i32;
    }

}


pub fn main() {

}