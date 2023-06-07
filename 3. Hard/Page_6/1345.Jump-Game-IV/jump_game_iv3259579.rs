// https://leetcode.com/problems/jump-game-iv/solutions/3259579/yet-another-bfs-rust-solution/
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        if arr.len() == 1 {
            return 0;
        }

        let mut indexes = HashMap::new();
        for (ix, &num) in arr.iter().enumerate() {
            indexes.entry(num).or_insert_with(|| vec![]).push(ix);
        }

        let mut visited = vec![false; arr.len()];

        let mut visited_current = HashSet::new();
        visited_current.insert(0);
        let mut visited_nxt = HashSet::new();

        let mut num_jumps = 0;

        loop {
            num_jumps += 1;

            for ix in visited_current.drain() {
                if ix != 0 && !visited[ix-1] {
                    visited_nxt.insert(ix-1);
                    visited[ix-1] = true;
                }
                if ix != arr.len() - 1 && !visited[ix+1] {
                    if ix + 1 == arr.len() - 1 {
                        return num_jumps;
                    }

                    visited_nxt.insert(ix+1);
                    visited[ix+1] = true;
                }
                for index in indexes.remove(&arr[ix]).into_iter().flatten() {
                    if index == ix || visited[index] {
                        continue;
                    }

                    if index == arr.len() - 1 {
                        return num_jumps;
                    }

                    visited_nxt.insert(index);
                    visited[index] = true;
                }
            }

            std::mem::swap(&mut visited_current, &mut visited_nxt);
        }
    }
}