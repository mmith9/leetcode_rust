struct Solution;

impl Solution {
    // even left, odd right
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let k = k as usize % mat[0].len();
        if k == 0 {return true}
        
        for line in mat.iter().step_by(2) {
            let mut rline = line.clone();
            rline.rotate_left(k);
            if rline != *line {return false}
        }
        for line in mat.iter().skip(1).step_by(2) {
            let mut rline = line.clone();
            rline.rotate_right(k);
            if rline != *line {return false}
        }
        return true;
    }
}



pub fn main() {

    let mat = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    let k = 4;
    let result = Solution::are_similar(mat, k);
    println!("{} false", result);

}