struct Solution;

impl Solution {
    pub fn score(cards: Vec<String>, x: char) -> i32 {
        let mut boths = 0;
        let mut lefts = [0;256];
        let mut rights = [0;256];
        let x = x as usize;
        for card in cards {
            let c0 = card.as_bytes()[0] as usize;
            let c1 = card.as_bytes()[1] as usize;
            lefts[c1] += (c0 == x) as i32;
            rights[c0] += (c1 == x) as i32;            
        }
        let mut boths = lefts[x]; lefts[x] = 0;rights[x] = 0;
        let mut rsum = 0; let mut rmax = 0;
        for &x in rights['a' as usize..='j' as usize].iter() {
            rsum +=x;
            rmax = rmax.max(x);
        }
        let mut lsum =0; let mut lmax=0;
        for &x in lefts['a' as usize..='j' as usize].iter() {
            lsum +=x;
            lmax = lmax.max(x);
        }        
        let lpairs = (lsum/2).min(lsum - lmax);
        let rpairs = (rsum/2).min(rsum - rmax);
        rsum -= rpairs*2; lsum -= lpairs*2;
        let bpairs = (rsum + lsum).min(boths);
        boths -= bpairs;
        return lpairs + rpairs + bpairs + (lpairs*2 +rpairs *2).min(boths/2);
    }
}


pub fn main() {}