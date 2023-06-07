// https://leetcode.com/problems/jump-game-iv/solutions/3260599/rust-bfs/
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        min_jumps(arr)
    }
}

use std::collections::{HashMap, VecDeque};

pub fn min_jumps(arr: impl AsRef<[i32]>) -> i32 {
    let arr = arr.as_ref();
    if arr.len() <= 1 {
        return 0;
    }

    let mut visited = vec![false; arr.len()];
    visited[0] = true;

    let mut index = HashMap::new();
    for (idx, key) in arr.iter().copied().enumerate() {
        index.entry(key).or_insert(vec![]).push(idx);
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);

    let mut dist = 0;
    while !queue.is_empty() {
        for _ in 0..queue.len() {
            let idx = queue.pop_front().unwrap();
            if idx == arr.len() - 1 {
                return dist;
            }

            if idx < arr.len() - 1 && !visited[idx + 1] {
                visited[idx + 1] = true;
                queue.push_back(idx + 1);
            }

            if idx > 0 && !visited[idx - 1] {
                visited[idx - 1] = true;
                queue.push_back(idx - 1);
            }

            let key = arr[idx];
            index.remove(&key).into_iter().flatten().for_each(|pos| {
                if !visited[pos] {
                    visited[pos] = true;
                    queue.push_back(pos);
                }
            });
        }

        dist += 1;
    }

    unreachable!()
}