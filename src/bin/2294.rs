struct Solution;

//hybrid approach
impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < 2000 {
            nums.sort_unstable();
            let mut res = 1;
            let mut limit = nums[0] +k;
            for num in nums {
                if num > limit {
                    res +=1;
                    limit = num + k;
                }
            }
            return res;
        } else {
            let mut freqs = [0u64;100001/64 +1];
            for num in nums {
                freqs[num as usize /64] |= 1 << (num % 64);
            }
            let mut res = 1;
            let start = freqs.iter().position(|x| *x>0).unwrap();            
            let mut limit = start as i32 *64 + (freqs[start].trailing_zeros() as i32) +k;
            let start = (start).max(1) -1;
            for (idx, mut f) in freqs.into_iter().enumerate().skip(start) {
                while f>0 {
                    let num = idx as i32 *64 + (f.trailing_zeros() as i32);
                    f ^= 1 << (f.trailing_zeros());
                    if num > limit {
                        res +=1;
                        limit = num+k;
                    }
                }
            }
            return res;
        }
    }
}


pub fn main() {
    
}