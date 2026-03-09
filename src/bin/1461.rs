struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let lens = s.len();
        let mut need_chars = (k - 1 + (1 << k)) as usize;
        if need_chars > lens {return false}

        let mut bitmap = vec![0u8;1 << k];
        let k = k as usize;
        let mut bin_code = 0 as usize;
        let bin_mask:usize = (1 << k) -1;

        for c in s.bytes().take(k) {
            bin_code = (bin_code << 1) + (c - b'0') as usize;
        }
        bitmap[bin_code] = 1;
        need_chars -= k-1;

        for (idx, c) in s.bytes().enumerate().skip(k) {
            bin_code = ((bin_code << 1) & bin_mask) + (c - b'0') as usize;
            
            need_chars -= 1 - bitmap[bin_code] as usize; 
            bitmap[bin_code] = 1;

            if need_chars == 0 {return true}
            if lens - idx < need_chars {return false}
        }
        return true
    }
}



pub fn main() {

    let s = "00110110".to_string(); let k=2;
    let result = Solution::has_all_codes(s, k);
    println!("{result} true");

    let s = "01100".to_string();
    let result = Solution::has_all_codes(s, 2);
    println!("{result} true");

    // let s = "0110".to_string(); let k = 2;
    // let result = Solution::has_all_codes(s, k);
    // println!("{result} false");

    // let s = "000011010111011001001111111001000100100100010100101100001101101101110001100100101111100111001001111001001010111010010101101001001110011100110101001001001000000110101001010011101100110110100010000".to_string();
    // let result = Solution::has_all_codes(s, 7);
    // println!("{result} false");

    // let s = "011101100101110101101000011111101011111101110100111100010000010110010010011100110001110010101101011010010001101111000111110000001010100101111001111010110001111011001110100010001111000111010001111100101011100001001011101100010101010110001011110101001101001001111101000100011101110100100100101101110000000110001011100100111111001000100100010011001000101101100010010010001111010111010011110111110001010100000110000111010110001100100110111000111010111000010100100100101011001111010110010101110101000011011101000110001001100111100011000100110010101100001111000100101001111001100001010100100100110100101100111000110010110101010110010110001111010110101111011011100111001010101001011000101101110100001110011110001011000011100011111001110011111101110001110010000111010011110001011010100101110010110110100011111011110010100011111000000001011100110000010101110110111".to_string();
    // let result = Solution::has_all_codes(s, 7);
    // println!("{result} false");


}