struct Solution;

const PRINT:bool = false;
impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        if PRINT {println!("---- case n:{n} k:{k}")}        
        let mut shards = (0..n as usize).collect::<Vec<usize>>();
        let mut fedges = vec![];
        let mut min_weight = i32::MAX;
        let mut edges_used = 0;

        for edge in edges {
            if edge[3] == 1 {
                if !Self::add_edge(&mut shards, edge[0] as usize,edge[1] as usize) {
                    if PRINT {println!("cycle on must edges")}
                    return -1}
                min_weight = min_weight.min(edge[2]);
                edges_used+=1;
            } else {
                fedges.push(edge);
            }
        }

        if edges_used +1 == n {return min_weight}

        fedges.sort_by_key(|x| x[2]);
        let mut w_opts = vec![];
        for edge in fedges.into_iter().rev() {
            if PRINT {println!("adding opt edge")}            
            if Self::add_edge(&mut shards, edge[0] as usize, edge[1] as usize) {
                if PRINT {println!("success")}
                w_opts.push(edge[2]);
                edges_used+=1;
                if edges_used +1 == n {break}
            }
        }
        if edges_used +1 != n {
            if PRINT {println!("not enough edges used {edges_used} but need {n}-1")}
            return -1
        }
        if w_opts.is_empty() {return min_weight}

        w_opts.sort();
        for i in 0..w_opts.len().min(k as usize) {w_opts[i as usize] *=2}
        return min_weight.min(*w_opts.iter().min().unwrap());

    }

    fn get_shard(mut shards:&mut Vec<usize>, node:usize) -> usize {
        let noderef = shards[node];
        if noderef != node {shards[node] = Self::get_shard(&mut shards, noderef)}
        return shards[node]
    }

    fn add_edge(mut shards:&mut Vec<usize>, node1:usize, node2:usize) -> bool {
        let mut ref1 = shards[node1]; let mut ref2 = shards[node2];
        if ref1 != node1 {ref1 = Self::get_shard(&mut shards, ref1)}
        if ref2 != node2 {ref2 = Self::get_shard(&mut shards, ref2)}
        if ref1 == ref2 {return false}
        let ref3 = ref1.min(ref2);
        shards[ref1] = ref3; shards[ref2] = ref3;
        return true;
    }

}






pub fn main() {
    println!("-----start of tests");    
    let n = 3;let edges = [[0,1,2,1].to_vec(),[1,2,3,0].to_vec()].to_vec();let k = 1;
    let expected = 2;
    let result = Solution::max_stability(n, edges, k);
    if result != expected {println!("{result} vs {expected} for {n} {k}")};

    let n = 3;let edges = [[0,1,4,0].to_vec(),[1,2,3,0].to_vec(),[0,2,1,0].to_vec()].to_vec(); let k = 2;
    let expected = 6;
    let result = Solution::max_stability(n, edges, k);
    if result != expected {println!("{result} vs {expected} for {n} {k}")};

    let n = 3;let edges = [[0,1,1,1].to_vec(),[1,2,1,1].to_vec(),[2,0,1,1].to_vec()].to_vec();let k = 0;
    let expected = -1;
    let result = Solution::max_stability(n, edges, k);
    if result != expected {println!("{result} vs {expected} for {n} {k}")};

    let n=5;let k = 2;
    let edges = [[2,0,68643,1].to_vec(),[1,0,31681,1].to_vec(),[4,2,44760,1].to_vec(),[3,4,19034,1].to_vec(),[1,4,24247,0].to_vec()].to_vec();
    let expected = 19034;
    let result = Solution::max_stability(n, edges, k);
    if result != expected {println!("{result} vs {expected} for {n} {k}")};

    let n = 4; let k =0;
    let edges = [[0,1,79543,1].to_vec(),[1,2,9966,1].to_vec(),[0,3,62157,0].to_vec(),[0,2,43960,1].to_vec()].to_vec();
    let expected = -1;
    let result = Solution::max_stability(n, edges, k);
    if result != expected {println!("{result} vs {expected} for {n} {k}")};

    let n = 5; let k =4;
    let edges = [[0,1,24819,0].to_vec(),[2,4,86210,1].to_vec(),[1,2,53407,0].to_vec(),[3,4,56877,0].to_vec(),[1,3,89383,0].to_vec()].to_vec();
    let expected = 49638;
    let result = Solution::max_stability(n, edges, k);
    if result != expected {println!("{result} vs {expected} for {n} {k}")};

    let n = 5; let k = 4;
    let edges = [[3,2,56447,1].to_vec(),[4,3,80497,1].to_vec(),[0,4,45565,1].to_vec(),[1,2,85317,1].to_vec(),[0,1,87891,0].to_vec(),[0,2,78889,0].to_vec(),[2,4,73816,0].to_vec()].to_vec();
    let expected = 45565;
    let result = Solution::max_stability(n, edges, k);
    if result != expected {println!("{result} vs {expected} for {n} {k}")};


    println!("-----End of tests");
}