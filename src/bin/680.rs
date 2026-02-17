struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let mut cut = -1;
        for (idx, (a,b)) in s.chars().zip(s.chars().rev()).enumerate() {
            if a!=b {cut = idx as i32; break}
        }
        if cut == -1 {return true}
        let cut1 = cut as usize;
        let cut2 = s.len() - cut1 -1;

        let mut s1:String = s.chars().rev().collect();
        let mut s2:String = s.clone();

        let ch = s1.remove(cut1);
        s2.remove(cut2);
        if s1 == s2 {return true};

        s1.insert(cut1 , ch);
        s2.insert(cut2, ch);
        s1.remove(cut2);
        s2.remove(cut1);

        if s1 == s2 {return true}

        return false;
    }
}


pub fn main() {

    let s = "aba".to_string();
    let result = Solution::valid_palindrome(s);
    println!("{} true", result);


    let s = "abca".to_string();
    let result = Solution::valid_palindrome(s);
    println!("{} true", result);


    let s = "abc".to_string();
    let result = Solution::valid_palindrome(s);
    println!("{} false", result);

}