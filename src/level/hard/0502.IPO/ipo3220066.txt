// https://leetcode.com/problems/ipo/solutions/3220066/rust-elixir-2-solutions/
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut indices: Vec<usize> = (0..profits.len()).collect();
        indices.sort_unstable_by_key(|&i| capital[i]);
        let mut i = 0;
        let mut heap = BinaryHeap::new();
        for _ in 0..k {
            while i < indices.len() && capital[indices[i]] <= w {
                heap.push(profits[indices[i]]);
                i += 1;
            }
            match heap.pop() {
                None => break,
                Some(p) => w += p,
            }
        }
        w
    }
}