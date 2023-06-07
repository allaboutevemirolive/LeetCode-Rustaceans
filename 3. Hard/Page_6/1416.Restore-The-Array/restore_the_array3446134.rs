// https://leetcode.com/problems/restore-the-array/solutions/3446134/rust-solution/
impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        for i in 0..s.len() {
            if s[i] == '0' {
                continue;
            }
            for j in i..s.len() {
                let num = s[i..=j].iter().collect::<String>().parse::<i64>().unwrap();
                if num > k as i64 {
                    break;
                }
                dp[j + 1] = (dp[j + 1] + dp[i]) % MOD;
            }
        }
        dp[s.len()] as i32    
    }
}