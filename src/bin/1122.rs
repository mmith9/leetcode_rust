

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut res = arr1.clone();
        let n2i:HashMap<i32, usize> = arr2.into_iter().enumerate().map(|(idx, x)| (x, idx)).collect();
        res.sort_by_key(|x| n2i.get(x).copied().unwrap_or((2000 + x) as usize ));
        res
    }
}



pub fn main() {
    let arr1 = vec![2,3,1,3,2,4,6,7,9,2,19];
    let arr2 = vec![2,1,4,3,9,6];
    let expected = vec![2,2,2,1,4,3,3,9,6,7,19];
    let result = Solution::relative_sort_array(arr1, arr2);
    if result == expected {println!("ok")}
    else {
        print!("{result:?}");
        print!("{expected:?}");
    }
}