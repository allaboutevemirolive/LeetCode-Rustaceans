// https://leetcode.com/problems/unique-paths-ii/solutions/2584039/rust-two-bottom-up-dp-solutions-with-comments/
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp_prev = obstacle_grid[0].iter().scan(1, |ok, cell| {
            *ok &= (*cell == 0) as i32;
            Some(*ok)
        }).collect::<Vec<_>>();
        let mut dp_curr = vec![0; n];
        for i in 1..m {
            dp_curr[0] = if obstacle_grid[i][0] == 0 { dp_prev[0] } else { 0 };
            for j in 1..n {
                dp_curr[j] = if obstacle_grid[i][j] == 0 { dp_curr[j-1] + dp_prev[j] } else { 0 };
            }
            std::mem::swap(&mut dp_prev, &mut dp_curr);
        }
        dp_prev[n-1]
    }
}