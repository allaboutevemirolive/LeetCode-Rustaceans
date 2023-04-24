// https://leetcode.com/problems/jump-game-iv/solutions/3259544/rust-solution/
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        use std::collections::*;
        let mut adj = HashMap::new();
        for i in 0..arr.len() {
            adj.entry(arr[i]).or_insert(vec![]).push(i as i32);
        }

        let mut visited = HashSet::new();
        let mut steps = 0;
        let mut curs = vec![0]; // current layer
        let n = arr.len() as i32;

        while !curs.is_empty() {
            let mut next = vec![]; // next layer
            for &v in &curs {
                if v == n - 1 {
                    return steps;
                }

                let map_key = arr[v as usize];
                for &u in &adj[&map_key] {
                    if !visited.contains(&u) {
                        visited.insert(u);
                        next.push(u);
                    }
                }
                adj.get_mut(&map_key).unwrap().clear();

                if (v - 1) >= 0 && !visited.contains(&(v - 1)) {
                    visited.insert(v - 1);
                    next.push(v - 1);
                }

                if v + 1 < n && !visited.contains(&(v + 1)) {
                    visited.insert(v + 1);
                    next.push(v + 1);
                }
            }
            curs = next;
            steps += 1;
        }
        -1 
    }
}