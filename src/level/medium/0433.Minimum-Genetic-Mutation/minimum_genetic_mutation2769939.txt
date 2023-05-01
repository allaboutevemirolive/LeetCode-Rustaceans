// https://leetcode.com/problems/minimum-genetic-mutation/solutions/2769939/rust-bfs-but-no-hash-set-no-heap-allocation-with-comments/
use std::collections::VecDeque;
use std::iter::once;

impl Solution {
    pub fn min_mutation(start: String, end: String, mut bank: Vec<String>) -> i32 {
        let dist = |a: &String, b: &String| a.bytes().zip(b.bytes()).filter(|(c, d)| !c.eq(d)).count();
        let mut mutations = 0;
        let mut q = once(start).collect::<VecDeque<_>>();
        while !q.is_empty() {
            for _ in 0..q.len() {
                let curr = q.pop_front().unwrap();
                if curr == end {
                    return mutations;
                }
                let mut i = 0;
                while i < bank.len() {
                    if dist(&curr, &bank[i]) == 1 {
                        q.push_back(bank.swap_remove(i));
                    } else {
                        i += 1;
                    }
                }
            }
            mutations += 1;
        }
        -1
    }
}