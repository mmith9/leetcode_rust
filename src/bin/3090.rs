struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let mut freqs = [0i32;26];
        let mut res = 0i32;
        let mut cur = 0i32;
        let mut left = s.chars();
        let mut right = s.chars();

        while let Some(c) = right.next() {
            let idx = (c as u8 - b'a') as usize;
            freqs[idx]+=1;
            cur +=1;
            while freqs[idx] >2 {
                let d = left.next().unwrap();
                freqs[(d as u8 -b'a') as usize] -=1;
                cur -=1;
            }
            res = res.max(cur);
        }
        return res;
    }
}


pub fn main() {
    let s = "bcbbbcba".to_string();
    println!("{} 4", Solution::maximum_length_substring(s));

}



