use std::vec;

struct Solution;
impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        if nums.len() < 4 {return false}
        if nums[0] >= nums[1] {return false}
        
        let mut lastn = nums[0];
        let mut stage = 0;
        for &num in nums.iter().skip(1) {
            if num == lastn {return false}
            if num < lastn {
                if stage %2 == 1 {stage +=1 }
            }
            else {
                if stage %2 == 0 {stage += 1}
            }
            if stage >3 {return false}
            lastn = num

        }
        return stage == 3
    }
}

fn main() {
    let ar1 = vec![1,3,5,4,2,6];
    let result = Solution::is_trionic(ar1);
    println!("{:?} True", result);

    let ar2 = vec![2,1,3];
    let result = Solution::is_trionic(ar2);
    println!("{:?} False", result);

    
}