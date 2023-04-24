// https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/1432221/rust-btreemap-solution/
use std::collections::BTreeMap;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let len = profit.len();
        let mut index = (0..len).collect::<Vec<_>>();
        index.sort_by_cached_key(|&i| end_time[i]);
        let mut btm = BTreeMap::new();
        btm.insert(0, 0);
        for &i in &index {
            if let Some((_, &p)) = btm.range(..=start_time[i]).last() {
                if p + profit[i] > *btm.iter().last().unwrap().1 {
                    btm.insert(end_time[i], p + profit[i]);
                }
            }
        }
        *btm.iter().last().unwrap().1
    }
}