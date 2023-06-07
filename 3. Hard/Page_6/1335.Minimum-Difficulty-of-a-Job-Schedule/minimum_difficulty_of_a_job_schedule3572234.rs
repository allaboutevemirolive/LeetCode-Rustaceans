// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/solutions/3572234/rust-bottom-up-approach/
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let d = d as usize;
        let n = job_difficulty.len();
        if n < d {
            return -1;
        }

        let mut dp = vec![vec![i32::MAX; d + 1]; n + 1];
        dp[n][d] = 0;
        for i in (0..n).rev() {
            dp[i][d] = dp[i + 1][d].max(job_difficulty[i]);
        }

        for day in (1..d as usize).rev() {
            for i in day - 1..n - (d - day) {
                let mut hardest = 0;
                for j in i..n - (d - day) {
                    hardest = hardest.max(job_difficulty[j]);
                    dp[i][day] = (hardest + dp[j + 1][day + 1]).min(dp[i][day]);
                }
            }
        }
        dp[0][1]
    }
}