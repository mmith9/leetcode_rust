struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]; matrix[0].len()];

        for line in &matrix {
            for (idx, num) in line.iter().enumerate() {
                res[idx].push(*num);
            }
        }
        return res;
    }
}


pub fn main() {



}