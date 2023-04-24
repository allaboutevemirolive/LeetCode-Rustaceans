// https://leetcode.com/problems/jump-game-iv/solutions/3257955/rust-elixir-hashing-bfs/
use std::collections::HashMap;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut hash = HashMap::new();
        for (i, &x) in arr.iter().enumerate() {
            hash.entry(x).or_insert(Vec::new()).push(i);
        }
        let mut visited = vec![false; arr.len()];
        visited[0] = true;
        
        let mut curr = vec![0];
        let mut next = Vec::new();
        for step in 0.. {
            for i in curr.drain(..) {
                if i == arr.len() - 1 {
                    return step;
                }
                for j in hash.remove(&arr[i])
                            .unwrap_or(Vec::new())
                            .into_iter()
                            .chain(vec![i.saturating_sub(1), i + 1].into_iter()) {
                    if !visited[j] {
                        next.push(j);
                        visited[j] = true;
                    }
                }
            }
            std::mem::swap(&mut curr, &mut next);
        }
        unreachable!()
    }
}