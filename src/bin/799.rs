
struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut line = vec![0f64;(query_row+1) as usize];
        line[0] = poured as f64;

        for row in 1..(query_row+1) as usize {
            let mut carry = 0f64;

            for glass in 0..row+1 {
                let next_carry = ((line[glass] -1.0 ) / 2.0).max(0.0);
                line[glass] = carry+next_carry;
                carry = next_carry;
            }
        }
        return line[query_glass as usize].min(1.0)
    }
}


pub fn main() {

    let (poured, row, glass) = (1,1,1);
    let result = Solution::champagne_tower(poured, row, glass);
    println!("{} 0", result);

    let (poured, row, glass) = (2,1,1);
    let result = Solution::champagne_tower(poured, row, glass);
    println!("{} 0.5", result);

    let (poured, row, glass) = (100000009,33,17);
    let result = Solution::champagne_tower(poured, row, glass);
    println!("{} 1.0", result);



}