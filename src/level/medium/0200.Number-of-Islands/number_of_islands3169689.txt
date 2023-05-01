// https://leetcode.com/problems/number-of-islands/solutions/3169689/rust-dfs/
impl Solution {

    fn dfs(grid: &mut Vec<Vec<char>>, row: usize, col: usize, m: usize, n: usize) {
        if grid[row][col] != '1' {
          return;
        } else {
          grid[row][col] = '#';
          if row > 0 { Self::dfs(grid, row - 1, col, m, n); }
          if row < m - 1 { Self::dfs(grid, row + 1, col, m, n); }
          if col > 0 { Self::dfs(grid, row, col - 1, m, n); }
          if col < n - 1 { Self::dfs(grid, row, col + 1, m, n); }
        }
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count: i32 = 0;
        let m: usize = grid.len();
        if m == 0 {
          return count;
        }
        let n: usize = grid[0].len();
        for row in 0..m {
          for col in 0..n {
            if grid[row][col] == '1' {
              Self::dfs(&mut grid, row, col, m, n);
              count += 1;
            }
          }
        }
        count
    }
}