struct Solution;
impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut res = 0;
        let mut ones:i32;
        let mut zeros:i32;

        for start in 0..s.len() {
            ones = 0; zeros = 0;
            for a in s[start..].chars() {
                if a == '1' {ones +=1}
                else {zeros +=1}
                if zeros >k && ones>k {zeros -=1; break}
            }
            res += ones + zeros;

        }

        return res
    }   
}

struct Solution2;
impl Solution2 {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut res = 0;
        let mut arr = [0,0];

        for start in 0..s.len() {
            arr[0] = 0; arr[1] = 0;
            for chr in s[start..].bytes() {
                arr[chr as usize & 1] += 1; // branchless
                if arr[0] >k && arr[1]>k {
                    arr[0] -= 1; 
                    break
                }
            }
            res += arr[0] + arr[1]
        }
        return res
    }
}

struct Solution3;
impl Solution3 {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut res = 0;
        let mut zeros:i32;
        let mut ones:i32;

        for start in 0..s.len() {
            ones = 0; zeros = 0;
            for chr in s[start..].bytes() {
                // branchless counting
                let bit = (chr & 1) as i32; 
                zeros += bit;
                ones += 1 - bit;
                if ones >k && zeros >k {
                    ones -=1;
                    break
                }
            }
            res += ones + zeros;
        }
        return res
    }
}


pub fn main() {}
