// https://leetcode.com/problems/cracking-the-safe/solutions/1542708/rust-eulerian-cycle-and-hierholzer-algorithm/
use std::collections::HashMap;
use std::collections::HashSet;

fn _gen_permutations(k: u8, p: usize, v: &mut Vec<u8>, accumulator: &mut Vec<Vec<u8>>) {
    for i in 0..k {
        v[p] = i;
        if p == v.len() - 1 {
            accumulator.push(v.clone());
        } else {
            _gen_permutations(k, p + 1, v, accumulator);
        }
    }
}

fn gen_permutations(n: u8, k: u8) -> Vec<Vec<u8>> {
    let mut accumulator: Vec<Vec<u8>> = Vec::with_capacity((k as u16).pow(n as u32) as usize);
    let mut v: Vec<u8> = vec![0; n as usize];
    _gen_permutations(k, 0, &mut v, &mut accumulator);
    accumulator
}

fn gen_graph(nodes: Vec<Vec<u8>>, k: u8) -> HashMap<Vec<u8>, Vec<Vec<u8>>> {
    let mut graph: HashMap<Vec<u8>, Vec<Vec<u8>>> = HashMap::new();
    for node in &nodes {
        let mut neighbors: Vec<Vec<u8>> = Vec::with_capacity(k as usize);
        let mut neighbor: Vec<u8> = Vec::with_capacity(node.len());
        for i in 1..node.len() {
            neighbor.push(node[i]);
        }
        neighbor.push(u8::MAX);
        for j in 0..k {
            neighbor[node.len() - 1] = j;
            neighbors.push(neighbor.clone());
        }
        graph.insert(node.clone(), neighbors);
    }
    graph
}

fn _hierholzer_walk(node: Vec<u8>, graph: &HashMap<Vec<u8>, Vec<Vec<u8>>>, visited: &mut HashSet<Vec<u8>>, path: &mut Vec<Vec<u8>>) {
    visited.insert(node.clone());
    for neigbour in graph.get(&node).unwrap() {
        if !visited.contains(neigbour) {
            _hierholzer_walk(neigbour.clone(), graph, visited, path);
        }
    }
    path.push(node);
}

fn eulerian_path(start: Vec<u8>, graph: &HashMap<Vec<u8>, Vec<Vec<u8>>>) -> Vec<Vec<u8>> {
    let mut path: Vec<Vec<u8>> = Vec::with_capacity(graph.len());
    let mut visited: HashSet<Vec<u8>> = HashSet::new();
    _hierholzer_walk(start.clone(), graph, &mut visited, &mut path);
    
    path
}


impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        if k == 1 {
            return vec![0; n as usize].into_iter().map(|x| x.to_string()).collect();
        }
        
        let mut nodes = gen_permutations(n as u8, k as u8);
        println!("nodes.len(): {:?}", nodes.len());
        println!("nodes: {:?}", nodes);
        
        let graph: HashMap<Vec<u8>, Vec<Vec<u8>>> = gen_graph(nodes, k as u8);
        println!("graph: {:?}", graph);
        
        let path = eulerian_path(vec![0; n as usize], &graph);
        println!("path: {:?}", path);
        
        let mut de_bruijn_sequence: String = path.into_iter().map(|v| v[v.len()-1].to_string()).collect();
        println!("de_bruijn_sequence: {:?}", de_bruijn_sequence);
        
        // Cyclic De Brujin sequence to a non-cyclic one.
        for c in de_bruijn_sequence[..n as usize - 1].to_string().chars() {
            de_bruijn_sequence.push(c);
        }
        
        de_bruijn_sequence
    }
}