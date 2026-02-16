

struct Solution;


impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        let parity = (arr.len() %2) as i32;
        let half = ((arr.len() - 1)/2) as i32;
        let mut factor = half+1;
        
        for (idx, num) in arr.iter().enumerate() {
            res += factor * num;
            factor += half -(idx as i32|parity);
        }
        return res
    }
}

pub fn main() {

    let arr = vec![1,2,3,4,5,6,7,8,9];
    let result = Solution::sum_odd_length_subarrays(arr);
    println!("{} 425", result);


    let arr = vec![1,4,2,5,3];
    let result = Solution::sum_odd_length_subarrays(arr);
    println!("{} 58", result);

    let arr = vec![1,2];
    let result = Solution::sum_odd_length_subarrays(arr);
    println!("{} 3", result);

    let arr = vec![10,11,12];
    let result = Solution::sum_odd_length_subarrays(arr);
    println!("{} 66", result);

    let arr = vec![7,6,8,6];
    let result = Solution::sum_odd_length_subarrays(arr);
    println!("{} 68", result);

    let arr = vec![7,6,8,6,7,7];
    let result = Solution::sum_odd_length_subarrays(arr);
    println!("{} 191", result);

    let arr = vec![6,9,14,5,3,8,7,12,13,1];
    let result = Solution::sum_odd_length_subarrays(arr);
    println!("{} 878", result);
}