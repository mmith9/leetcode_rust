struct Solution;


impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        if a.len() < b.len() {return Self::add_binary(b,a)}

        let mut res:Vec<char> = Vec::new();
        let mut ones = 0;
        let mut iterb = b.chars().rev();
        for c in a.chars().rev() {
            ones += (c == '1') as u8;
            ones += (iterb.next() == Some('1')) as u8;
            res.push((ones%2 + b'0') as char);
            ones = ones/2;
        }
        if ones == 1 {res.push('1')}
        return res.iter().rev().collect()
    }
}


pub fn main() {

    let a = "11".to_string(); let b = "1".to_string();
    let result = Solution::add_binary(a, b);
    println!("{} 100", result);
    
    let a = "1010".to_string(); let b = "1011".to_string();
    let result = Solution::add_binary(a, b);
    println!("{} 10101", result);
    
}