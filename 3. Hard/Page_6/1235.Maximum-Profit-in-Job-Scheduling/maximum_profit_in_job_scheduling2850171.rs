// https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/2850171/rust-10-lines-solution-btreemap-dp-sorting/
use std::collections::BTreeMap;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut best = 0;
        let mut jobs: Vec<_> = start_time.into_iter().zip(end_time).zip(profit).collect();
        let mut dp = BTreeMap::new();

        jobs.sort_by_key(|&((_, e), _)| e);

        for ((s, e), p) in jobs {
            let rez = dp.range(..=s).next_back().map(|(_, v)| v).unwrap_or(&0) + p;
            best = best.max(rez);
            dp.insert(e, best);
        }

        best
    }
}