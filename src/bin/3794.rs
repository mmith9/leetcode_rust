struct Solution;


impl Solution {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let head:String = s.chars().take(k as usize).collect();
        let head:String = head.chars().rev().collect();
        let mut res = s.clone();
        res.replace_range(0..k as usize, &head);
        return res
    }
}

//simpler
struct Solution2;
impl Solution2 {
    pub fn reverse_prefix(s: String, k: i32) -> String {
        let head:String = s[..k as usize].chars().rev().collect();
        let res = head + &s[k as usize..];
        
        return res
    }
}


pub fn main() {
    Solution::reverse_prefix(String::from("a text for a text"), 3);

}