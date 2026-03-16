struct Solution;

struct Edge {
    to:usize,
}

struct Node {
    edges:Vec<Edge>,
}

impl Node {
    pub fn new() -> Self {
        Self {edges:Vec::new()}
    }
}

struct Graph {
    frees: Vec<usize>,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            frees: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self) {
        let idx;
        if let Some(idx_) = self.frees.pop() { idx = idx_}
        else {idx = self.nodes.len();}
        let node = Node::new();
        
        if idx == self.nodes.len() {self.nodes.push(node)}
        else {self.nodes[idx] = node};
    }

    pub fn add_edge(&mut self, node1:usize, node2:usize) {
        let edge = Edge{to:node2};
        self.nodes[node1].edges.push(edge);
        let edge = Edge{to:node1};
        self.nodes[node2].edges.push(edge);
    }

    pub fn remove_edge(&mut self, node1:usize, node2:usize) {
        self.nodes[node1].edges.retain(|x| x.to != node2);
        self.nodes[node2].edges.retain(|x| x.to != node1);

    }

    pub fn find_disjoints_and_cycles(&self, node:usize, norecu:bool) -> (Vec<usize>, bool) {
        let mut visited = vec![];
        let mut queue = vec![];
        queue.push((node,usize::MAX));

        while !queue.is_empty() {
            let mut nextq = vec![];
            for node_p in queue.into_iter() {
                let node = node_p.0;
                let parent = node_p.1;
                visited.push(node);
                for edge in self.nodes[node].edges.iter() {
                    if edge.to == parent {continue}
                    if visited.contains(&edge.to) {return (vec![], true)}
                    nextq.push((edge.to, node));
                }
            }
            queue = nextq;
        }

        let mut disjoints = vec![];
        for x in 0..self.nodes.len() {
            if !visited.contains(&x) {disjoints.push(x)}
        }

        if !norecu {
            let mut dis_to_check = disjoints.clone();
            let mut try_disjoints;let mut cyclic;
            while !dis_to_check.is_empty() {
                let node = dis_to_check.pop().unwrap();
                (try_disjoints, cyclic) = Self::find_disjoints_and_cycles(&self, node, true);
                if cyclic {return (vec![], true)}
                dis_to_check.retain(|x| try_disjoints.contains(x));
            }
        }
        return (disjoints, false);
    }


}

const PRINT:bool = false;

// bad solution, goes tle 
// totally unecessary graph implementation
impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        if PRINT {
            println!("----");
            println!("n {n}, k{k}");
            for edge in edges.iter() {
                println!("{edge:?}");
            }
        }
        if n as usize > edges.len() +1 {return -1}
        if n <= edges.iter().filter(|x| x[3] == 1).count() as i32 {return -1}

        let mut graph = Graph::new();
        for _ in 0..n {graph.add_node()}

        let mut fedges = vec![vec![0i32;4];0];
        let mut min_must_weight = i32::MAX;
        // edge == [node1, node2, weight, mandatory]
        for edge in edges.into_iter() {
            if edge[3] == 1 {
                graph.add_edge(edge[0] as usize, edge[1] as usize);
                min_must_weight = min_must_weight.min(edge[2]);
            } else {
                fedges.push(edge);
            }
        }
        let mut disjoints; let mut cyclic;
        (disjoints, cyclic) = graph.find_disjoints_and_cycles(0, false);
        if PRINT {println!("first check cyclic:{cyclic}, disjoints {disjoints:?}");}
        if cyclic {return -1}
        let mut weights_used = vec![];
        fedges.sort_by_key(|x| x[2]);

        while !disjoints.is_empty() {
            let mut node1;let mut node2;let mut weight;
            loop {
                if fedges.is_empty() {return -1}
                let edge = fedges.pop().unwrap();
                node1=edge[0] as usize; node2=edge[1] as usize;weight=edge[2];
                if disjoints.contains(&node1) || disjoints.contains(&node2) {break}
            }
        if PRINT {println!("trying edge {node1} {node2} {weight}")}
            graph.add_edge(node1, node2);
            let try_disjoints;
            (try_disjoints, cyclic) = graph.find_disjoints_and_cycles(0, false);
        if PRINT {println!("result cyclic:{cyclic}, disjoints {try_disjoints:?}")}
            if cyclic {
                graph.remove_edge(node1, node2);
            }
            else {
                weights_used.push(weight);
                disjoints = try_disjoints;
            }
        }
        weights_used.sort();
        for idx in 0..weights_used.len().min(k as usize) {
            weights_used[idx] *= 2;
        }
        weights_used.push(min_must_weight);
        if PRINT {println!("{weights_used:?}")}
        return *weights_used.iter().min().unwrap()

    }
}



pub fn main() {
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


}