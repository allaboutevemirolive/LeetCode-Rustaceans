// https://leetcode.com/problems/jump-game-iv/solutions/3259137/rust-bfs-solution/
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut indices = HashMap::new();
        for (i, &n) in arr.iter().enumerate() {
            indices.entry(n).or_insert(vec![]).push(i);
        }
        let goal = arr.len() - 1;
        let mut seen = vec![false; arr.len()];
        let mut q = VecDeque::new();
        q.push_back(0);
        seen[0] = true;
        let mut step = 0;
        while !q.is_empty() {
            for _ in 0..q.len() {
                let i = q.pop_front().unwrap();
                if i == goal {
                    return step;
                }
                if i > 0 && !seen[i - 1] {
                    seen[i - 1] = true;
                    q.push_back(i - 1);
                }
                if i < goal && !seen[i + 1] {
                    seen[i + 1] = true;
                    q.push_back(i + 1);
                }
                if let Some(adj) = indices.get_mut(&arr[i]) {
                    while let Some(j) = adj.pop() {
                        q.push_back(j);
                        seen[j] = true;
                    }
                }
            }
            step += 1;
        }
        -1
    }
}