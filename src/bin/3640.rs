
struct Solution;

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let (mut direction, mut changes) =(0,0); //track direction and bounces
        let (mut up2, mut uptail) = (0,0); //track first uphill - best tail sum
        let (mut uphead, mut uphead_best)=(0,0); //track second uphill - best head sum
        let mut midsum = 0; // downhill sum is always on full slope
        let mut res:i64 = i64::MIN;
        let mut lastn:i64 = nums[0].into();
                
        for num in nums {
            let num:i64 = num.into();

            if num == lastn {(direction, changes) = (0,0)} // flat, reset the sequence

            else if num < lastn { // going down
                if direction < 0 {midsum += num} // just continue down

                else if direction >0 { // switch from up to down
                    direction = -1;
                    changes += 1;
                    midsum = uptail + up2 + num + lastn; // collect accrued tail
                }
            }

            else { // going up
                if direction == 0 { // initialize sequence
                    uptail = 0;
                    up2 = lastn;
                    direction = 1;
                }
                else if direction > 0 { 
                    uptail += up2;
                    up2 = lastn;
                    if uptail < 0 {uptail = 0} // discard negative tail, keep only two required values - up2 and lastn

                    if changes >=2 {
                        uphead += num;
                        uphead_best = uphead_best.max(uphead);
                        res = res.max(midsum + uphead_best);
                    }
                }
                else { //were going down, bouncing up
                    uphead = num;
                    uphead_best = num;
                    direction = 1;
                    changes += 1;
                    uptail = 0;
                    up2 = lastn;

                    if changes >=2 {
                        res = res.max(midsum + uphead_best);
                    }
                }

            } 
            lastn = num;
               
        }
        return res
        
    }
}



pub fn main() {

/* 
    let nums = vec![0,-2,-1,-3,0,2,-1];
    let result = Solution::max_sum_trionic(nums);
    println!("-4 got {}", result);


    let nums = vec![1,4,2,7];
    let result = Solution::max_sum_trionic(nums);
    println!("14 got {}", result);

    let nums = vec![-432,186,-568,390];
    let result = Solution::max_sum_trionic(nums);
    println!("-424 got {}", result);
*/
    let nums = vec![159,208,-920,-460,295];
    let result = Solution::max_sum_trionic(nums);
    println!("-718 got {}", result);

    let nums = vec![286,528,-900,327,536,625,547,997];
    let result = Solution::max_sum_trionic(nums);
    println!("3032 got {}", result);


}