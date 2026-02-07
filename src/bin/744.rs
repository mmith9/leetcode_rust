//744. Find Smallest Letter Greater Than Target
struct Solution;
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for &letter in &letters {
            if letter > target {
                return letter;
            }
        }
        return letters[0]
    }
}


fn main() {
    
    let result = Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a');
    println!("{:?}", result);
}   


