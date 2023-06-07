// https://leetcode.com/problems/count-different-palindromic-subsequences/solutions/816523/rust-translated/
impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = s.len();
        let v = s.as_bytes();
        let mut dp = vec![vec![0i64; n]; n];
        for len in 1..=n {
            for i in 0..n + 1 - len {
                let j = i + len - 1;
                if i == j {
                    dp[i][j] = 1;
                } else if i == j - 1 {
                    dp[i][j] = 2;
                } else {
                    if v[i] != v[j] {
                        dp[i][j] = dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1];
                    } else {
                        let c = v[i];
                        let mut left = i + 1;
                        let mut right = j - 1;
                        while left < j && v[left] != c {
                            left += 1;
                        }
                        while right > i && v[right] != c {
                            right -= 1;
                        }
                        if left == j {
                            dp[i][j] = dp[i + 1][j - 1] * 2 + 2;
                        } else if left == right {
                            dp[i][j] = dp[i + 1][j - 1] * 2 + 1;
                        } else {
                            dp[i][j] = dp[i + 1][j - 1] * 2 - dp[left + 1][right - 1];
                        }
                    }
                }
                dp[i][j] = if dp[i][j] < 0 {
                    dp[i][j] + MOD
                } else {
                    dp[i][j] % MOD
                };
            }
        }
        dp[0][n - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_palindromic_subsequences() {
        assert_eq!(
            Solution::count_palindromic_subsequences("bccb".to_string()),
            6
        )
    }

    #[test]
    fn test_count_palindromic_subsequences_02() {
        assert_eq!(
            Solution::count_palindromic_subsequences(
                "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".to_string()
            ),
            104860361
        )
    }
}