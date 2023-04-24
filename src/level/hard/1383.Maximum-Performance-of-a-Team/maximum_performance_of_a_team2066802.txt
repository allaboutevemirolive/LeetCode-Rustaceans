// https://leetcode.com/problems/maximum-performance-of-a-team/solutions/2066802/rust-solution/
use std::collections::*;
use std::cmp::*;

const MOD:usize = 1_000_000_007;
impl Solution {
  pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut memo = vec![(0,0);n];
    for i in 0..n {
      memo[i] = (efficiency[i] as usize, speed[i] as usize);
    }
    memo.sort();

    let mut heap = BinaryHeap::new();
    let mut tot = 0usize;
    let mut result = 0usize;
    let mut last = memo[n-1].0;
    while let Some((ev, sv)) = memo.pop() {
      if ev != last {
        last = ev;
      }

      tot += sv;
      heap.push(Reverse(sv));

      if k < heap.len() {
        let min = heap.pop().unwrap().0;
        tot -= min;
      }
      result = std::cmp::max(result, tot * last);
    }
    
    (result % MOD) as i32
  }
}