// https://leetcode.com/problems/restore-the-array/solutions/3446021/iterative-dp-solution-in-rust/
impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        const MOD:i32 = 1_000_000_007;
        let n = s.len();
        let k_10 = k / 10;
        let mut num_ways_dp = vec![0; n + 1];
        num_ways_dp[n] = 1;
        let bytes = s.as_bytes();
        for (i, c) in bytes.iter().copied().enumerate().rev() {
            let mut num_ways = 0;
            if c!=b'0' {
                let mut num = 0;
                for idx in i..n {
                    if num <= k_10 {
                        let d = (bytes[idx] - b'0') as i32;
                        num = num * 10  + d;
                        if num <= k {
                            num_ways = (num_ways + num_ways_dp[idx+1]) % MOD;
                            continue;
                        }
                    }
                    break;
                }
            }
            num_ways_dp[i] = num_ways;
        }

        num_ways_dp[0]
    }
}