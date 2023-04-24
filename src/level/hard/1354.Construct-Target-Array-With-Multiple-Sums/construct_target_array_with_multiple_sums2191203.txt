// https://leetcode.com/problems/construct-target-array-with-multiple-sums/solutions/2191203/rust-binaryheap/
use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut sum = 0;
        let mut pq: BinaryHeap<_> = target.into_iter().map(|t| {sum += t as i64; t as i64}).collect();
        loop {
            match pq.pop().unwrap() {
                1 => return true,
                max => {
                    let p = sum - max;
                    if p == 0 || p >= max {
                        return false;
                    }
                    let x = match max % p { 0 => p, x => x };
                    pq.push(x);
                    sum -= max - x;
                },
            }
        }
    }
}