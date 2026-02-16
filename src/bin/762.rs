// use std::collections::HashSet;

struct Solution;

impl Solution {
    #[inline(never)]
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        // limited number of primes in i32
        // even more limited by constraints 1 <= left <= right <= 10**6 < 2**18
        
        // checking 8 values is actually faster than hashset
        // let primes = vec![2,3,5,7,11,13,17,19];
        let prime_bits: [i32; 20] = [0,0,1,1,0,1,0,1,0,0,0,1,0,1,0,0,0,1,0,1];
        // let mut prime_bits = [0u8;32];
        // for bits in primes {
        //     prime_bits[bits] = 1;
        // }
//        let primes:HashSet<u32> = HashSet::from([2,3,5,7,11,13,17,19]);
        let mut res = 0;
        for num in left..=right {
            // if primes.contains(&(num.count_ones() as u8)) {
                //  res+=1;}
            res += prime_bits[num.count_ones() as usize]; 
            }
        return res;
    }
}


pub fn main() {

    let left = 10; let right = 15;
    let result = Solution::count_prime_set_bits(left, right);
    println!("{} 5", result)

}