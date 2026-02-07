struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut res = nums.len();
        let mut idx = 0;

        while idx < nums.len() {
            if nums[idx] == val {
                res -=1;
                nums.swap_remove(idx);
            }
            else {idx+=1}
        }
        return res as i32
    }
}



pub fn main() {

}