// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/642552/rust-recursive-dp-solution/
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let bytes = s.bytes().collect::<Vec<u8>>();
        let mut dp = vec![vec![0; s.len()]; s.len()];
        min_steps_to_palindrome(&bytes, 0, s.len() - 1, &mut dp)
    }
}

fn min_steps_to_palindrome(s: &Vec<u8>, start: usize, end: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
    if start >= end {
        return 0;
    }
    
    if dp[start][end] != 0 {
        return dp[start][end];
    }

    dp[start][end] = if s[start] == s[end] {
        min_steps_to_palindrome(s, start + 1, end - 1, dp)
    } else {
        1 + min_steps_to_palindrome(s, start + 1, end, dp).min(min_steps_to_palindrome(s, start, end - 1, dp))
    };
    dp[start][end]
}