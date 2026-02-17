struct Solution;


impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut got_zero = false;

        for c in s.chars() {
            if c == '1' && got_zero {return false}
            got_zero = c == '0';
        }
        return true;
    }
}


pub fn main() {


}