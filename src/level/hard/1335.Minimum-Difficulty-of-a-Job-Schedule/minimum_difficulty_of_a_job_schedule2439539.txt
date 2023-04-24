// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/solutions/2439539/rust-dp/
// dp[i][j]: the solution to the sub-problem of job_difficulty[0..=i] and j + 1 days
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;
        let mut dp = vec![vec![i32::MAX; d]; n];
        
        for k in 0..d {
            if k == 0 {
                let mut mx = 0;
                for i in 0..n {
                    mx = mx.max(job_difficulty[i]); 
                    dp[i][0] = mx; 
                }
            } else {
                for i in k..n {
                    let mut mx = 0;
                    for j in (k..=i).rev() {
                        mx = mx.max(job_difficulty[j]);
                        if dp[j - 1][k - 1] == i32 { break; }
                        dp[i][k] = dp[i][k].min(mx + dp[j - 1][k - 1]);
                    } 
                }
            } 
        }
        if dp[n - 1][d -  1] {  return -1 }
        dp[n - 1][d - 1]
    }
}