// https://leetcode.com/problems/ipo/solutions/3220085/rust-greedy-o-n-log-n/
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut q = BinaryHeap::new();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_unstable_by_key(|&i| capital[i]);
        let mut i = 0;
        let mut fund = w;
        let mut k = k as usize;
        while k > 0 {
            while i < n {
                let o = order[i];
                if capital[o] > fund {
                    break;
                }
                q.push(profits[o]);
                i += 1;
            }
            if let Some(p) = q.pop() {
                fund += p;
            } else {
                break;
            }
            k -= 1;
        }
        fund
    }
}