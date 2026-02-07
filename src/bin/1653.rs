
struct Solution;

// branchless 1 pass, 0 ms
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let (mut tmp_a, mut all_a, mut res) = (0,0,0);

        for c in s.bytes() {
            all_a += (c&1) as i32;
            tmp_a += (c&2) as i32 -1;
            res = res.min(tmp_a);
        }
        res = (res + all_a).min(s.len() as i32 - all_a);
        return res;
    }
}

// // branchless 0 ms
// impl Solution {
//     pub fn minimum_deletions(s: String) -> i32 {
//         let mut all_a= 0;
//         for c in s.bytes(){all_a += (c&1) as usize }
//         let mut res = all_a.min(s.len() - all_a);

//         for c in s.bytes() {
//             all_a = all_a + (c & 2) as usize -1;
//             res = res.min(all_a);
//         }
      
//         return res as i32;
//     }
// }

        // for c in s.bytes() {
        //     let bit = (c&1) as usize;
        //     all_a -= bit;
        //     count_b += 1 - bit;
        //     res = res.min(all_a + count_b);

// struct Solution2;
// impl Solution2 {
//     pub fn minimum_deletions(s: String) -> i32 {
//         let mut b:usize = 0;
//         let mut res: usize = 0;

//         for c in s.bytes() {
//             let bit = (c&1) as usize;
//             b += 1 - bit;
//             res = (res + bit).min(b);
//             }
    
//         return res as i32;
//         }
//     }



pub fn main() {
    let s = "aababbab".to_string();
    let result = Solution::minimum_deletions(s);
    println!("{} expected 2",result);
    let s = "bbaaaaabb".to_string();
    let result = Solution::minimum_deletions(s);
    println!("{} expected 2",result);

}