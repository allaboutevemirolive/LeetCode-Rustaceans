// https://leetcode.com/problems/jump-game-iv/solutions/3260182/rust-100-time-100-memory-dfs-solution/
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return 0;
        }
        let mut dupls: HashMap<i32, Vec<usize>> = HashMap::new();
        arr.iter().enumerate().for_each(|(i, &val)| {
            dupls.entry(val).or_default().push(i);
        });
        Self::bfs(&arr, &mut dupls, &mut vec![false; arr.len()], arr.len() - 1)
    }

    pub fn bfs(arr: &Vec<i32>, neighbours: &mut HashMap<i32, Vec<usize>>, visited: &mut [bool], target: usize) -> i32 {
        let mut count = 0;
        visited[0] = true;
        let mut start = vec![0];
        loop {
            let mut next_level = vec![];
            for &node in start.iter() {
                if node == target {
                    return count;
                }
                if node > 0 && !visited[node - 1] {
                    visited[node - 1] = true;
                    next_level.push(node - 1)
                }
                if !visited[node + 1] {
                    visited[node + 1] = true;
                    next_level.push(node + 1)
                }
                if let Some(neigh) = neighbours.get(&arr[node]){
                    neigh.iter().for_each(|&val| {
                        if !visited[val] {
                            visited[val] = true;
                            next_level.push(val)
                        }
                    });
                    neighbours.remove(&arr[node]);
                }
            }
            count += 1;
            start = next_level;
        }
    }
}