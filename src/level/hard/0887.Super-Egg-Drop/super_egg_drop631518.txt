// https://leetcode.com/problems/super-egg-drop/solutions/631518/rust-dp-0ms-100/
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let n_i = n as usize;
        let mut dp = Vec::with_capacity(n_i + 1);
        for i in 0..=n {
            dp.push(i);
        }
        let mut last_dp = dp.clone();

        for _ in 2..=k {
            dp.clear();
            dp.push(0);
            for i in 1..=n_i {
                if dp[i - 1] >= n {
                    break;
                }
                dp.push(1 + dp[i - 1] + last_dp[i - 1]);
            }
            last_dp[..dp.len()].copy_from_slice(&dp);
        }
        dp.len() as i32 - 1
    }
}