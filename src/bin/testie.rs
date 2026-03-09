struct Solution;

use gcd::Gcd;
impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut lastn = -1;
        let mut curseq = 0; let mut bestseq = 0;
        let mut cursum = 0; let mut bestsum = 0;
        for &num in &nums {
            if num == lastn +1 {
                cursum += num;
                curseq += 1;
            } else {
                curseq = 1;
                cursum = num;
            }
            if curseq > bestseq {
                bestseq = curseq;
                bestsum = cursum;
            }
        }
        while nums.contains(&bestsum) {bestsum+=1}
        return bestsum;


        let a = 1u32;
        let b = 1u32;
        let c = a.gcd(b);
    }
}


pub fn main() {

}