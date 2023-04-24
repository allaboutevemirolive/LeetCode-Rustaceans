// https://leetcode.com/problems/jump-game-iv/solutions/3261270/rust-yabfs-but-beats-100/
use std::collections::{HashMap,HashSet,VecDeque};
use std::iter::once;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut similar_jumps = arr
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut h, (idx, x)| {
                h.entry(x).or_insert(vec![]).push(idx);
                h
            });
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        let mut visited = HashSet::new();
        visited.insert(0);
        while let Some((curr, dist)) = q.pop_front() {
            if curr == arr.len() - 1 {
                return dist;
            }
            similar_jumps.get(&arr[curr]).map(|neighs| {
                neighs
                    .iter()
                    .chain(once(&(curr - 1)).chain(once(&(curr + 1))).filter(|&&neigh| neigh < arr.len()))
                    .for_each(|&neigh| {
                        if !visited.contains(&neigh) {
                            q.push_back((neigh, dist + 1));
                            visited.insert(neigh);
                        }
                    })
            });
            // don't need to iterate anymore on the same valued cells
            similar_jumps.entry(&arr[curr]).and_modify(|v| v.clear());
        }
        0
    }
}