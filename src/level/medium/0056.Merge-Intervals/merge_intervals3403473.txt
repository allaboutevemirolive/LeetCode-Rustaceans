// https://leetcode.com/problems/merge-intervals/solutions/3403473/rust-solution/
use std::cmp;
impl Solution {

    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Sort intervals
        intervals.sort_unstable_by_key(|interval1| interval1[0]);

        // merge interval
        let mut merged_intervals: Vec<Vec<i32>> = Vec::new();
        let mut index = 0;
        for interval in intervals {
            if merged_intervals.is_empty() || merged_intervals.last().unwrap()[1] < interval[0] {
                merged_intervals.push(interval);
            } else {
                let mut last_interval = merged_intervals.pop().unwrap();
                last_interval[1] = cmp::max(last_interval[1], interval[1]);
                merged_intervals.push(last_interval);
            }
        }              
        merged_intervals
    }
}