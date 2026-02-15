struct Solution;

impl Solution {
    #[inline(never)]
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut res:Vec<String> = Vec::new();
        let r1 = "qwertyuiop";
        let r2 = "asdfghjkl";
        let r3 = "zxcvbnm";

        for word in &words {

            let mut chars = word.chars();
            let c = chars.next().unwrap().to_ascii_lowercase();
            let group:&str;

            if r1.contains(c) {group = r1}
            else if r2.contains(c) {group = r2}
            else {group = r3}
            
            let mut passes = true;
            for c in chars {
                if ! group.contains(c.to_ascii_lowercase()) {passes = false; break}
            }
            if passes {res.push(word.clone())}
                        
        }
        return res
    }
}

pub fn main() {

    let words = vec!["Hello".to_string(),"Alaska".to_string(),"Dad".to_string(),"Peace".to_string()];
    let result = Solution::find_words(words);
    println!("{:?}", result);

}