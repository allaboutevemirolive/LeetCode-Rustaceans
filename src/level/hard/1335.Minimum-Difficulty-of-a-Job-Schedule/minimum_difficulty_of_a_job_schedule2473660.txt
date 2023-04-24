// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/solutions/2473660/rust-top-down-and-bottom-up-dp-with-comments/
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let (n, d) = (job_difficulty.len(), d as usize);
        if d > n {
            return -1;
        }
        let mut dp = vec![vec![i32::MAX; d + 1]; n + 1];
        dp[n][d] = 0;
        for j in (0..d).rev() {
            for i in (j..=n-(d-j)).rev() {
                let mut day_difficulty = 0;
                for k in i..=n-(d-j) {
                    day_difficulty = day_difficulty.max(job_difficulty[k]);
                    dp[i][j] = dp[i][j].min(day_difficulty.saturating_add(dp[k+1][j+1]));
                }
            }
        }
        dp[0][0]
    }
}