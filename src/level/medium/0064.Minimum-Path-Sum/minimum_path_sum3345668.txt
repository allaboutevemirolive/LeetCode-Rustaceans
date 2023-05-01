// https://leetcode.com/problems/minimum-path-sum/solutions/3345668/rust-dp-solution/
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut f = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    f[j] = grid[i][j];
                } else if i == 0 {
                    f[j] = f[j - 1] + grid[i][j];
                } else if j == 0 {
                    f[j] = f[j] + grid[i][j];
                } else {
                    f[j] = f[j].min(f[j - 1]) + grid[i][j];
                }
            }
        }

        f[n - 1]
    }
}