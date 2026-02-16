struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut res = 0;
        let mut num = n;
        while num >0 {
            res += num &1;
            num = num >>1;
        }
        return res
    }
}



pub fn main() {
    let result = Solution::hamming_weight(11);
    println!("{} 3", result);
}