// https://leetcode.com/problems/restore-the-array/solutions/3446778/rust-1d-dp/
impl Solution {
    const MODULO: usize = 1_000_000_007;
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let k = k as usize;
        let n = s.len();

        let s_bytes = s.as_bytes();

        let mut dp = vec![0; n + 1];
        dp[n] = 1;

        for i in (0..n).rev() {
            let mut num: usize = 0;
            for j in i..n {
                num = num * 10 + (s_bytes[j] - b'0') as usize;
                if num > k || s_bytes[i] == b'0' {
                    break;
                }
                dp[i] = (dp[i] + dp[j + 1]) % Self::MODULO;
            }
        }

        dp[0] as i32
    }
}