// https://leetcode.com/problems/restore-the-array/solutions/3446940/clean-rust-1d-dp-solution/
impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let modulus = 1_000_000_007;

        let n = s.len();
        let mut dp = vec![0; n+1];

        dp[n] = 1;
        for i in (0..n).rev() {
            if s.as_bytes()[i] == b'0' {
                continue;
            }

            let mut cnt = 0;
            let mut num = 0i64;

            for j in i..n {
                num = num * 10 + (s.as_bytes()[j] - b'0') as i64;
                if num > k as i64 {
                    break;
                }
                cnt = (cnt + dp[j+1]) % modulus;
            }

            dp[i] = cnt;
        }

        dp[0] % modulus
    }
}