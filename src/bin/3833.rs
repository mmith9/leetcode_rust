struct Solution;

impl Solution {
    pub fn dominant_indices(nums: Vec<i32>) -> i32 {
        let mut total = nums.iter().sum();
        let mut cnt = nums.len() as i32;
        let mut res = 0;
        for &num in nums[..nums.len()-1].iter() {
            cnt-=1;
            total -= num;
            res += (num*cnt > total) as i32;
        }
        return res
    }
}


pub fn main() {

}