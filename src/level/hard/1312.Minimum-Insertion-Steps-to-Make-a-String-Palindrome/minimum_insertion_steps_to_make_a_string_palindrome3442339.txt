// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/solutions/3442339/rust-dp-top-down/
impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        fn dfs(i: usize, j: usize, s: &[u8], table: &mut Vec<Vec<i32>>) -> i32 {
            if i >= j { return 0 }
            if table[i][j] != -1 { return table[i][j] }
            if s[i] == s[j] { return dfs(i + 1, j - 1, s, table) }
            table[i][j] = dfs(i + 1, j, s, table).min(dfs(i, j - 1, s, table)) + 1;
            table[i][j]
        }

        let s = s.as_bytes();
        let n = s.len();
        let mut table = vec![vec![-1; n]; n];

        dfs(0, n - 1, &s, &mut table)
    }
}