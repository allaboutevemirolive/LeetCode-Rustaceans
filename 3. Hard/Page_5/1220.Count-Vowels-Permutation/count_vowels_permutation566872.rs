// https://leetcode.com/problems/count-vowels-permutation/solutions/566872/rust-8ms-easy-solution-with-state-map/
const MOD: u64 = 1_000_000_007;
const STATE_MAP: [[i32; 5]; 5] = [
    [0, 1, 1, 0, 1],
    [1, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 1, 0],
];

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut dp = vec![[0_u64; 5]; n as usize];
        dp[0] = [1; 5];

        for i in 1..n as usize {
            for j in 0..5 {
                for g in 0..5 {
                    if STATE_MAP[j][g] != 0 {
                        dp[i][j] = (dp[i][j] + dp[i - 1][g]) % MOD;
                    }
                }
            }
        }

        let sum: u64 = dp[n as usize - 1].iter().sum();
        (sum % MOD) as i32
    }
}