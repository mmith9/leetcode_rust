struct Solution;



impl Solution {
    pub fn largest_submatrix(mut matrix: Vec<Vec<i32>>) -> i32 {
        for j in 1..matrix.len() {
            let (top, down) = matrix.split_at_mut(j);
            let lastline = top.last().unwrap();
            let line = &mut down[0];
            for (cur, prev) in line.iter_mut().zip(lastline.iter()) {
                *cur *= prev +1;
            }
        }
        let mut res = 0;

        // slower actually.
        
        // let mut bucketmap = vec![];
        // let mut buckets = vec![0i32;matrix.len()+1];
        // for row in matrix.into_iter() {
        //     for x in row.into_iter() {
        //         let cnt = buckets[x as usize];
        //         if cnt == 0 {
        //             bucketmap.push(x as usize)
        //         }
        //         buckets[x as usize] = cnt+1;
        //     }
        //     bucketmap.sort();
        //     let mut cols = 0;
        //     for &bucket in bucketmap.iter().rev() {
        //         cols += buckets[bucket];
        //         res = res.max(cols * bucket as i32);
        //         buckets[bucket] = 0;
        //     }
        //     bucketmap.clear();
        // }



        for mut row in matrix.into_iter() {
            row.sort_unstable();
            let mut cols = 1;
            for col in row.into_iter().rev() {
                res = res.max(col *cols);
                cols +=1;                
            }
        }
        return res;
    }
}



pub fn main() {
    let mut example = 0;
    println!("testing");

    let matrix_ = [[0,0,1],[1,1,1],[1,0,1]]; let expected = 4;
    example +=1;
    let mut  matrix = vec![];
    for x in matrix_ {matrix.push(x.to_vec())};
    let result = Solution::largest_submatrix(matrix);
    if expected != result {
        println!("error, got {result} expected {expected} in case {example}");
    }

 


}