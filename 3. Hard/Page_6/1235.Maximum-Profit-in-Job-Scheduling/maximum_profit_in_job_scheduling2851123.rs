// https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/2851123/rust-15ms/
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<_> = start_time
            .into_iter()
            .zip(end_time.iter())
            .zip(profit.iter())
            .map(|((a, b), c)| (a, *b, *c))
            .collect();

        jobs.sort();

        let mut dp = vec![-1; jobs.len()];

        fn dfs(i: usize, jobs: &Vec<(i32, i32, i32)>, dp: &mut Vec<i32>) -> i32 {
            if i >= jobs.len() { return 0; }
            if dp[i] >= 0      { return dp[i]; }

            let k = jobs.partition_point(|&job| job.0 < jobs[i].1);
            dp[i] = dfs(i + 1, jobs, dp).max(jobs[i].2 + dfs(k, jobs, dp));
            return dp[i];
        }

        return dfs(0, &jobs, &mut dp);
    }
}