struct Solution;

// // 4bit optimization means 16 bytes for lookup
// // can implement 8bit with 256 lookup 
// impl Solution {
//     pub fn reverse_bits(n: i32) -> i32 {
//         let lookup:[u8;16] = [0,8,4,12,2,10,6,14,1,9,5,13,3,11,7,15];
//         let mut res: [u8; 4] = n.to_le_bytes();
        
//         for b in &mut res {
//             let lo = *b & 15;
//             let hi = *b >> 4;
//             *b = (lookup[lo as usize] << 4) | lookup[hi as usize];
//         }
//         return i32::from_be_bytes(res);
//     }
// }

// s

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut n = n as u32;

        n = ((n >>  1) & 0x55555555) | ((n & 0x55555555) <<  1); // 16x '01'
        n = ((n >>  2) & 0x33333333) | ((n & 0x33333333) <<  2); // 8x  '0011'
        n = ((n >>  4) & 0x0f0f0f0f) | ((n & 0x0f0f0f0f) <<  4); // 4x  '00001111'
        n = ((n >>  8) & 0x00ff00ff) | ((n & 0x00ff00ff) <<  8); // 2x  '0000000011111111'
        n = ((n >> 16)             ) | ((n             ) << 16); // no need to mask as halves do no overlap anymore
        // can be extended to 64 bit with n >> 32. And adding mask 0xffffffff to the line above

        return n as i32
    }
}

pub fn main() {

    let result = Solution::reverse_bits(43261596);
    println!("{} 964176192", result);

    let result = Solution::reverse_bits(2147483644);
    println!("{} 1073741822", result);

}


