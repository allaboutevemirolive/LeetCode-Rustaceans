// https://leetcode.com/problems/unique-paths-iii/solutions/2973682/rust-dfs-solution/
const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<i32>>, m: usize, n: usize, res: &mut i32, x: usize, y: usize, count: i32, zeros: i32) {
            if grid[x][y] == 2 && count == zeros {
                *res += 1;
            }

            let value = grid[x][y];
            let next_count = if value == 0 { count + 1 } else { count };
            grid[x][y] = -1;

            for dir in DIRS {
                let nx = dir.0 + x as i32;
                let ny = dir.1 + y as i32;

                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] != -1 {
                    dfs(grid, m, n, res, nx as usize, ny as usize, next_count, zeros);
                }
            }

            grid[x][y] = value;
        }

        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let mut res = 0;
        let mut zeros = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    zeros += 1;
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    dfs(&mut grid, m, n, &mut res, i, j, 0, zeros);
                }
            }
        }

        res
    }
}