// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/solutions/704231/rust-dp-dfs-8ms/
impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let h = matrix.len();
        if h == 0 { return 0; }
        let w = matrix[0].len();
        let mut dp = vec![vec![-1; w]; h];

        fn dfs(matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, h: usize, w: usize, r: usize, c: usize) -> i32 {
            if dp[r][c] < 0 {
                dp[r][c] = 0;
                let x = matrix[r][c];
                let min_neb = [if r >= 1 && matrix[r - 1][c] > x { dfs(matrix, dp, h, w, r - 1, c) } else { 0 },
                    if r < h - 1 && matrix[r + 1][c] > x { dfs(matrix, dp, h, w, r + 1, c) } else { 0 },
                    if c >= 1 && matrix[r][c - 1] > x { dfs(matrix, dp, h, w, r, c - 1) } else { 0 },
                    if c < w - 1 && matrix[r][c + 1] > x { dfs(matrix, dp, h, w, r, c + 1) } else { 0 }, ].iter().cloned().max().unwrap();
                dp[r][c] = min_neb + 1;
            }
            dp[r][c]
        }

        let mut ans = 0;
        for ir in 0..h {
            for ic in 0..w {
                ans = ans.max(dfs(&matrix, &mut dp, h, w, ir, ic));
            }
        }

        ans
    }
}