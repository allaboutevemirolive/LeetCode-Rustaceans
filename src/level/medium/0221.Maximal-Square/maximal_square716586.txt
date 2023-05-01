// https://leetcode.com/problems/maximal-square/solutions/716586/rust-dp-4ms/
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let (h, w) = (matrix.len() + 1, matrix[0].len() + 1);
        let mut dp: Vec<Vec<i32>> = vec![vec![0; w]; h];
        let mut ans = 0;
        for ir in 1..h {
            for ic in 1..w {
                if matrix[ir - 1][ic - 1] == '1' {
                    let x = dp[ir - 1][ic].min(dp[ir][ic - 1]).min(dp[ir - 1][ic - 1]) + 1;
                    dp[ir][ic] = x;
                    ans = ans.max(x);
                }
            }
        }

        ans * ans
    }
}