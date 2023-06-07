// https://leetcode.com/problems/ipo/solutions/3220087/rust-sorting-and-mx-heap/
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut projects = vec![];

        for (&p, &c) in profits.iter().zip(capital.iter()) {
            projects.push((p, c))
        }

        projects.sort_by(|a, b| a.1.cmp(&b.1));

        let mut i = 0;

        for _ in 0..k {
            while i < projects.len() && projects[i].1 <= w {
                heap.push(projects[i].0);
                i += 1;
            }
            if let Some(v) = heap.pop() { w += v }
        }

        w
    }
}