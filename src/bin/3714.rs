
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        if s.len() < 3 {return s.len() as i32}
        let mut res = 0;

        let mut noc = 0;    // a-b
        let mut nob = 0;    // a-c
        let mut noa = 0;    // b-c
        let mut nocs: HashMap<i32, i32> = HashMap::new();
        let mut nobs: HashMap<i32, i32> = HashMap::new();
        let mut noas: HashMap<i32, i32> = HashMap::new();
        nocs.insert(0, -1);
        nobs.insert(0, -1);
        noas.insert(0, -1);

        let mut ab_difs: HashMap<(i32, i32), i32> = HashMap::new();
        ab_difs.insert((0,0), -1);

        let mut lastc = '-';
        let mut conqq = 0;

        for (idx, c) in s.chars().enumerate() {
            let idx = idx as i32;
            if c == lastc {conqq +=1; res = res.max(conqq)}
            else {conqq = 1; lastc = c}

            if c == 'a' {
                noc +=1;
                nob +=1;
                noas = HashMap::new();
            }
            else if c == 'b' {
                noa +=1;
                noc -=1;
                nobs = HashMap::new();                
            }
            else {
                noa -=1;
                nob -=1;
                nocs = HashMap::new();                
            }

            let key = (nob, noa);
            if let Some(&firstidx) = ab_difs.get(&key) {
                res = res.max(idx - firstidx);
            } else {
                ab_difs.insert(key, idx);
            }
            
            if let Some(&firstidx) = noas.get(&noa) {
                res = res.max(idx - firstidx);
            } else {
                noas.insert(noa, idx);
            }

            if let Some(&firstidx) = nobs.get(&nob) {
                res = res.max(idx - firstidx);
            } else {
                nobs.insert(nob, idx);
            }

            if let Some(&firstidx) = nocs.get(&noc) {
                res = res.max(idx - firstidx);
            } else {
                nocs.insert(noc, idx);
            }
        }
        return res;        
    }
}






pub fn main() {
    let s= "aabcc".to_string();
    let result = Solution::longest_balanced(s);
    println!("{} 3", result);

    let s = "abbac".to_string();
    let result = Solution::longest_balanced(s);
    println!("{} 4", result);

    let s = "bac".to_string();
    let result = Solution::longest_balanced(s);
    println!("{} 3", result);


}
