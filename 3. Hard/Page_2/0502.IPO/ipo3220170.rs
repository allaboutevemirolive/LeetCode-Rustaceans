// https://leetcode.com/problems/ipo/solutions/3220170/rust-greedy-with-heap-solution/
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut w = w;
        let mut k = k;
        let n = profits.len();
        let mut cp: Vec<(i32, i32)> = capital.into_iter().zip(profits.into_iter()).collect();

        cp.sort_by(|a, b| {
            a.0.cmp(&b.0)
        });

        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut i = 0;

        while k > 0 {
            while i < n && cp[i].0 <= w {
                max_heap.push(cp[i].1);
                i += 1;
            }

            if !max_heap.is_empty() {
                w += max_heap.pop().unwrap();
            }

            k -= 1;
        }

        w
    }
}