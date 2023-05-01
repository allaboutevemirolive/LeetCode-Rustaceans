// https://leetcode.com/problems/longest-palindromic-substring/solutions/3151010/rust-dp-solution/
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut longest = 0;
        let mut f = vec![vec![0; n]; n];

        for len in 1..n + 1 {
            for i in 0..n - len + 1 {
                let j = i + len - 1;

                if i == j {
                    f[i][j] = 1;
                } else if i + 1 == j {
                    f[i][j] = if chars[i] == chars[j] { 2 } else { 0 };
                } else {
                    if chars[i] == chars[j] {
                        f[i][j] = if f[i + 1][j - 1] > 0 { f[i + 1][j - 1] + 2 } else { 0 };
                    }
                }

                if f[i][j] > longest {
                    longest = f[i][j];
                    start = i;
                }
            }
        }

        s[start..start + longest].to_string()
    }
}