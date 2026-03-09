struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut res = s.clone();

        // lefts stores unmatched lefts of ranges to be
        let mut lefts:Vec<(i32,usize)> = Vec::new();
        // heap stores complete ranges
        let mut heap:Vec<(i32, usize, usize)> = vec![(-1,0,0)];

        let mut height = 0;
        for (idx, c) in s.chars().enumerate() {
            if c == '1' {
                lefts.push((height, idx));
                height +=1;
            } else {
                if  height == heap[heap.len()-1].0 {
                    if height == heap[heap.len()-2].0 {
                        //got atleast two consecutive specials on same level
                        Solution::sort_strings(height, &mut heap, &mut res);
                    } else {
                        //got only one, just pop it
                        heap.pop();
                    }
                // } else {// nothing on heap, nothing to do}
                }
                height -=1;
                // close range on the current level
                let left = lefts.pop().unwrap().1;
                heap.push((height, left, idx));
                
            }
        }

        // one last sort
        if heap.len() >2 {Solution::sort_strings(height, &mut heap, &mut res)}
        return res;
    }


    fn sort_strings(height:i32, heap:&mut Vec<(i32, usize, usize)>, res:&mut String)  {
        let mut allleft:usize =0;
        let mut strs:Vec<String> = Vec::new();

        let allright = heap[heap.len()-1].2;
        while heap[heap.len()-1].0 == height { 
            let (_, left, right) = heap.pop().unwrap();
            let substr:String = res[left..=right].to_string();
            strs.push(substr);
            allleft = left;
        }
        strs.sort_by(|a,b| b.cmp(a)); // sort by reverse order
        let glued = strs.join("");

        // not needeed for ascii strings, just for sake of learning
        let start = res.char_indices().nth(allleft).unwrap().0;
        let end = res.char_indices().nth(allright).unwrap().0;

        res.replace_range(start..=end, glued.as_str());
    }

}



pub fn main() {
    let s = "11011000".to_string();
    let result = Solution::make_largest_special(s);
    let answer = "11100100".to_string();
    println!("{result} {}", answer == result);

}