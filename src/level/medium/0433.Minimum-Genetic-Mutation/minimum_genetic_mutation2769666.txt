// https://leetcode.com/problems/minimum-genetic-mutation/solutions/2769666/rust-bfs-0ms/
use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        // visited set used to store already seen mutations and avoid repetittion
        let mut visited = HashSet::new();
        // queue for our bfs
        let mut queue = VecDeque::new();
        queue.push_back((start.clone(), 0));
        visited.insert(start);
        
        let genes = vec!["A", "C", "G", "T"];
        
        while let Some((gene, steps)) = queue.pop_front() {
            if gene == end {
                return steps;
            }
            
            // changing each index of our gene string with one of the following options ['A', 'C', 'G', 'T'] and adding it to the queue if not visited
            for i in 0..gene.len() {
                for j in 0..genes.len() {
                    let mut mutation = gene.clone();
                    mutation.replace_range(i..i+1, genes[j]);
                    
                    if bank.contains(&mutation) && !visited.contains(&mutation) {
                        queue.push_back((mutation.clone(), steps+1));
                        visited.insert(mutation);
                    }
                }
            }
        }
        -1
    }
}