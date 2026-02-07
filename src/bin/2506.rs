use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        
        let mut dict:HashMap<u32, i32> = HashMap::new();

        for word in words {
            let mut wordmask = 0u32;

            for ord in word.bytes() {
                wordmask |= 1u32 << (ord-b'a')
            }

            dict.entry(wordmask).and_modify(|x| *x+=1).or_insert(1);
        }
        let mut res = 0;

        for count in dict.values() {
            res += count*(count-1) /2;
        }
        
        return res
    }
}


pub fn main(){
    let words = vec!["aabb".to_string(),"ab".to_string(), "ba".to_string()];
    let result = Solution::similar_pairs(words);
    println!("{} expected 3", result)


}