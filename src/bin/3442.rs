struct Solution;
impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freqs = [0u32; 26];
        for ord in s.bytes() {
            freqs[(ord -b'a') as usize] +=1;
        }

        let mut fodd = 0;
        let mut feven = u32::MAX;
        for x in freqs {
            if (x&1) != 0 {fodd = fodd.max(x)}
            else if x != 0 {feven = feven.min(x)}
        }
        
        return (fodd - feven) as i32
    }
}


pub fn main() {
    let s = "aaaaabbc".to_string();
    let result = Solution::max_difference(s);
    println!("{} expected 3", result)
}