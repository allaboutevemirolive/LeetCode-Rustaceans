// https://leetcode.com/problems/maximum-performance-of-a-team/solutions/1253056/rust-binaryheap-solution/
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DIV: i64 = 1_000_000_007;

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let mut v = efficiency
            .iter()
            .zip(speed.iter())
            .map(|(&e, &s)| (Reverse(e as i64), s as i64))
            .collect::<Vec<_>>();
        v.sort_unstable();
        let mut bh = BinaryHeap::with_capacity(k as usize);
        let mut sum = 0;
        let mut answer = 0;
        for &(Reverse(e), s) in &v {
            sum += s;
            bh.push(Reverse(s));
            if bh.len() > k as usize {
                if let Some(Reverse(min)) = bh.pop() {
                    sum -= min;
                }
            }
            answer = answer.max(sum * e);
        }
        (answer % DIV) as i32
    }
}