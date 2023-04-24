// https://leetcode.com/problems/maximum-profit-in-job-scheduling/solutions/2849785/rust-solution/
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut v: Vec<_> = end_time
            .into_iter()
            .zip(start_time)
            .zip(profit)
            .map(|((x, y), z)| (x, y, z))
            .collect();
        v.sort_unstable();
        let mut dp = Vec::with_capacity(v.len() + 1);
        dp.push(0);
        for (i, &(_, s, p)) in v.iter().enumerate() {
            let j = v[..i].partition_point(|&(e, _, _)| e <= s);
            dp.push(dp[i].max(dp[j] + p));
        }
        *dp.last().unwrap()
    }
}