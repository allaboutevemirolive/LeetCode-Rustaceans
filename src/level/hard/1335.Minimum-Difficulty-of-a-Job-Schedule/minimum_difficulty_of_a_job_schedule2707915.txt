// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/solutions/2707915/rust-dp-using-hint-2/
impl Solution {
    pub fn min_difficulty(jobs: Vec<i32>, d: i32) -> i32 {
        if d as usize > jobs.len() {
            return -1;
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![-1; d as usize]; jobs.len()];
        dp[0][0] = jobs[0];
        for i in 1..jobs.len() {
            dp[i][0] = dp[i - 1][0].max(jobs[i]);
        }
        Self::min_diff_util(&jobs, jobs.len() - 1, d as usize - 1, &mut dp)
    }

    fn min_diff_util(jobs: &[i32], i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if dp[i][j] == -1 {
            dp[i][j] = i32::MAX;
            let mut right = i32::MIN;
            for mid in (j..=i).rev() {
                right = right.max(jobs[mid]);
                dp[i][j] = dp[i][j].min(Self::min_diff_util(jobs, mid - 1, j - 1, dp) + right);
            }
        }
        dp[i][j]
    }
}