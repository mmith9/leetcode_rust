struct Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut hi = -1;
        let mut second = -1;
        for c in s.bytes() {
            if b'0' <= c && c <= b'9' {
                let val = (c - b'0') as i32;
                if val > hi {
                    second = second.max(hi);
                    hi = val;
                }
                else if val < hi {second=second.max(val)}
            }
        }
        return second;        
    }
}



pub fn main() {
    let s = "dfa12321afd".to_string();
    let result = Solution::second_highest(s);
    println!("{} 2", result);

    let s = "abc1111".to_string();
    let result = Solution::second_highest(s);
    println!("{} -1", result);


}