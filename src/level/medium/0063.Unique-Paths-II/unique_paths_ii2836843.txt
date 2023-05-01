// https://leetcode.com/problems/unique-paths-ii/solutions/2836843/rust-in-place/
impl Solution {
    pub fn unique_paths_with_obstacles(mut obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if *obstacle_grid.first().unwrap().first().unwrap() == 1
            || *obstacle_grid.last().unwrap().last().unwrap() == 1
        {
            return 0;
        }
        obstacle_grid[0][0] = -1;

        for row in 0..obstacle_grid.len() {
            for col in 0..obstacle_grid[0].len() {
                //
                if obstacle_grid[row][col] == 1 {
                    continue;
                }
                if row > 0 && obstacle_grid[row - 1][col] < 0 {
                    obstacle_grid[row][col] += obstacle_grid[row - 1][col];
                }
                if col > 0 && obstacle_grid[row][col - 1] < 0 {
                    obstacle_grid[row][col] += obstacle_grid[row][col - 1];
                }
            }
        }
        -*obstacle_grid.last().unwrap().last().unwrap()
    }
}
