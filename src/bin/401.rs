struct Solution;

use std::sync::{OnceLock, Mutex};
use std::collections::HashMap;

static CACHE: OnceLock<Mutex<HashMap<i32, Vec<i32>>>> = OnceLock::new();

fn get_cache() -> &'static Mutex<HashMap<i32, Vec<i32>>> {
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut res:Vec<String> = Vec::new();
        if turned_on > 8 {return res}

        let cache = get_cache();
        let mut bit2num = cache.lock().unwrap();

        if bit2num.get(&0i32).is_none() {
            println!("building cache");
            for x in 0i32..9 {bit2num.insert(x, Vec::new());}
            for x in 0i32..60 {
                let bits = x.count_ones() as i32;
                let v = bit2num.get_mut(&bits).unwrap();
                v.push(x);
            }
        }
        
        let limit = turned_on.min(3);
        for h_bits in 0..=limit {
            let m_bits = turned_on - h_bits;
            for h in &bit2num[&h_bits] {
                if *h > 11 {break}
                for m in &bit2num[&m_bits] {
                    res.push(format!("{h}:{m:0>2}"));
                }
            }
        }

        return res;        
    }
}


pub fn main() {

    println!("{:?}", Solution::read_binary_watch(0));
    println!("{:?}", Solution::read_binary_watch(1));


}





