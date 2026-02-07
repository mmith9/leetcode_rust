struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let str1 = &strs[0];
        let mut limit = str1.len();

        for str2 in strs.iter().skip(1) {
            let mut cur = 0;
            for (x, y) in str1.chars().zip(str2.chars()) {
                if x != y {break}
                cur+=1;
                if cur >= limit {break}
            }
            if cur < limit {
                limit = cur;
                if limit == 0 {return String::new()}
                }
        }
        return str1[0..limit].to_owned()
    }
}

pub fn main() {


}
