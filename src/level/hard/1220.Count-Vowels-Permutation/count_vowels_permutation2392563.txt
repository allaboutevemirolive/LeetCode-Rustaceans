// https://leetcode.com/problems/count-vowels-permutation/solutions/2392563/rust-dynamic-programming/
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        let mut dp = vec![vec![0; 5]; n as usize];
        dp[0] = vec![1; 5];
        
        let modu = 10i64.pow(9) + 7;
        for i in 1..n as usize {
            for j in 0..5 {
                let vowel = vowels[j];
                match j {
                    0 => {
                        dp[i][j] = dp[i-1][1] + dp[i-1][4] + dp[i-1][2];
                    },
                    1 => {
                        dp[i][j] = dp[i-1][0] + dp[i-1][2];
                    },
                    2 => {
                        dp[i][j] = dp[i-1][1] + dp[i-1][3];
                    },
                    3 => {
                        dp[i][j] = dp[i-1][2];                      
                    },
                    _ => {
                        dp[i][j] = dp[i-1][2] + dp[i-1][3];
                    }
                }
                dp[i][j] %= modu;
            }
        }
        
        let mut count = 0;
        for c in &dp[n as usize-1] {
            count += *c;
        }
        
        (count % modu) as i32
    }
}