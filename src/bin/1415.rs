struct Solution;
 
impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let n = n as usize;let mut k = k-1; //0 indexing
        let letters = ['a','b','c'];
        let mut bin3 = vec![];

        while k>0 && k!=2 {bin3.push(k&1); k = k>>1}

        if k == 2 {
            if bin3.len() +1 == n {bin3.push(k)}
            else {bin3.push(0);bin3.push(1)}
        }
        if n < bin3.len() {return "".to_string()}

        bin3.reverse();
        let mut res = vec![];
        for x in (0..n).into_iter(){res.push(letters[x &1])}

        let mut dif = n - bin3.len();
        let mut prevc;
        if dif != 0 {
            prevc = res[dif-1]
        } else {
            let subs = match bin3[0] {
                1 => {'b'}
                2 => {'c'}
                _ => {'a'}
            };
            res[0] = subs; prevc = subs; 
            dif+=1; bin3.remove(0);
        }

        for idx in 0..bin3.len() {
            let curbin = bin3[idx];
            let nextc = match (prevc, curbin) {
                ('a', 0) => {'b'}
                ('b', 0) => {'a'}
                ('c', 0) => {'a'}
                ('a', 1) => {'c'}
                ('b', 1) => {'c'}
                _ => {'b'}
            };
            res[idx+dif] = nextc;
            prevc = nextc;
        }

        return res.into_iter().collect::<String>();
    }
}

pub fn main() {
    let mut n; let mut k;let mut res;let mut expected;

    n=1;k=3;
    res = Solution::get_happy_string(n, k);
    expected = "c".to_string();
    if res!=expected {println!("n={n} k={k} got >{res}< need >{expected}<")}
    
    n=1;k=4;
    res = Solution::get_happy_string(n, k);
    expected = "".to_string();
    if res!=expected {println!("n={n} k={k} got >{res}< need >{expected}<")}

    n=3;k=9;
    res = Solution::get_happy_string(n, k);
    expected = "cab".to_string();
    if res!=expected {println!("n={n} k={k} got >{res}< need >{expected}<")}

}