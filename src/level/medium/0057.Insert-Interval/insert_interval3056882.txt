// https://leetcode.com/problems/insert-interval/solutions/3056882/rust-solution-easy/
use std::cmp::{max, min};

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![new_interval];
        }
        let mut result: Vec<Vec<i32>> = vec![];
        for interval in intervals {
            if !new_interval.is_empty() && interval[1] < new_interval[0] {
                result.push(interval.clone());
                continue;
            }
            if new_interval.is_empty() {
                result.push(interval);
                continue;
            }
            if interval[0] > new_interval[1] {
                result.push(new_interval.clone());
                new_interval.clear();
                result.push(interval);
                continue;
            }
            new_interval[0] = min(interval[0], new_interval[0]);
            new_interval[1] = max(interval[1], new_interval[1]);
        }
        if !new_interval.is_empty() {
            result.push(new_interval);
        }
        result
    }
}