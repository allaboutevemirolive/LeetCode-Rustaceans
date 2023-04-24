// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/solutions/2708959/rust-dp-solution/
impl Solution {
	pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
		let (n, d) = (job_difficulty.len(), d as usize);
		if n < d as usize { return -1; }
		let mut dp = vec![vec![i32::MAX;d + 1];n + 1];
		dp[0][0] = 0;
		for i in 1..=n {
			let mut max = 0;
			for j in (0..i).rev() {
				max = max.max(job_difficulty[j]);
				for k in 0..d {
					if dp[j][k] < i32::MAX {
						dp[i][k + 1] = dp[i][k + 1].min(dp[j][k] + max);
					}
				}
			}
		}
		dp[n][d]
	}
}