// https://leetcode.com/problems/maximal-square/solutions/600212/rust-dp-solution/
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let (n, m) = (matrix.len(), matrix[0].len());
        let mut dp: Vec<Vec<i32>> = vec![vec![0; m + 1]; n + 1];

        for y in 1..=n {
            for x in 1..=m {
                if matrix[y - 1][x - 1] == '1' {
                    dp[y][x] = dp[y - 1][x].min(dp[y][x - 1]).min(dp[y - 1][x - 1]) + 1;
                }
            }
        }

        dp.into_iter()
            .map(|row| row.into_iter().max().unwrap())
            .max()
            .unwrap()
            .pow(2)
    }
}