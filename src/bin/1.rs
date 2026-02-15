struct Solution;

impl Solution {
    #[inline(never)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = Vec::new();
        for x in 0..nums.len()-1 {
            let need = target - nums[x];

            for y in x+1..nums.len() {
                if nums[y] == need {
                    res.push(x.try_into().unwrap()); 
                    res.push(y.try_into().unwrap()); 
                    return res
                }
            }
        }
        return res
    }
}

pub fn main() {
    let arr1 = vec![1,2,3,4,5];
    let result = Solution::two_sum(arr1, 7);
    print!("got {:?}", result)

}