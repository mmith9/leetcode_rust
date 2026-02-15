struct Solution;


impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        if s.len() < 3 {return s.len() as i32}

        let lens = s.len();
        let mut res = 0;

        for i in 0..lens {
            let chars_left = lens - i;
            if res >= chars_left {return res as i32}
            let mut freqs = [0usize;26];
            let mut freqhi:usize = 1;
            let mut distincts = 0;
            let mut hi_count: usize = 0;
            let mut req_to_fill: usize = 0;

            for (j, c) in s[i..].chars().enumerate() {
                let cc:usize = c as usize - 'a' as usize;

                freqs[cc] +=1 ;

                if freqs[cc] == 1 {
                    distincts +=1;
                    req_to_fill += freqhi;
                    if req_to_fill > chars_left {break}
                }

                if freqs[cc] > freqhi {
                    req_to_fill += distincts;
                    if req_to_fill > chars_left {break}
                    freqhi = freqs[cc];
                    hi_count = 1;
                    if distincts == 1{
                        res = res.max(1 + j);
                    }
                }
                else if freqs[cc] == freqhi {
                    hi_count +=1;
                    if hi_count == distincts {
                        res = res.max(1 + j);
                    }
                }
            }
        }
        return res as i32;
    }
}


pub fn main() {

    let s = "abbac".to_string();
    let result = Solution::longest_balanced(s);
    println!("{} 4", result);

    let s = "bbbb".to_string();
    let result = Solution::longest_balanced(s);
    println!("{} 4", result);



}