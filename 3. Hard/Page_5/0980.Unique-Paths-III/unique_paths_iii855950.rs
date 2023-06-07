// https://leetcode.com/problems/unique-paths-iii/solutions/855950/rust-solution/
impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut pos = (0, 0);
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                match grid[i][j] {
                    0 => count += 1,
                    1 => pos = (i, j),
                    _ => {}
                }
            }
        }
        Solution::helper(&mut grid, pos, count + 1)
    }
    fn helper(grid: &mut Vec<Vec<i32>>, pos: (usize, usize), count: usize) -> i32 {
        if grid[pos.0][pos.1] == 2 {
            return if count == 0 { 1 } else { 0 };
        }
        grid[pos.0][pos.1] = 1;
        let ret = if pos.0 > 0 && grid[pos.0 - 1][pos.1].abs() != 1 {
            Solution::helper(grid, (pos.0 - 1, pos.1), count - 1)
        } else {
            0
        } + if pos.1 > 0 && grid[pos.0][pos.1 - 1].abs() != 1 {
            Solution::helper(grid, (pos.0, pos.1 - 1), count - 1)
        } else {
            0
        } + if pos.0 < grid.len() - 1 && grid[pos.0 + 1][pos.1].abs() != 1 {
            Solution::helper(grid, (pos.0 + 1, pos.1), count - 1)
        } else {
            0
        } + if pos.1 < grid[0].len() - 1 && grid[pos.0][pos.1 + 1].abs() != 1 {
            Solution::helper(grid, (pos.0, pos.1 + 1), count - 1)
        } else {
            0
        };
        grid[pos.0][pos.1] = 0;
        ret
    }
}