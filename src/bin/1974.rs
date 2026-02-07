struct Solution;
impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut res:i32 = word.len() as i32;
        let mut last = 'a' as u8;
        let mut diff:i32;
        for a in word.bytes() {
            diff = last.abs_diff(a) as i32;
            res += diff.min(26 - diff);
            last = a;
            }
        
        return res
    }
}

pub fn main() {
    let word = "asdasdfsadfdha".to_string();
    let result = Solution::min_time_to_type(word);
    println!("{}", result);

}

