// https://leetcode.com/problems/unique-paths-iii/solutions/1180684/rust-backtracking-solution/
use std::collections::HashSet;

impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let cur = (|| {
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 1 {
                        return (i, j);
                    }
                }
            }
            panic!();
        })();
        let target = {
            let mut res = 0;
            for i in 0..m {
                for j in 0..n {
                    if grid[i][j] == 0 {
                        res += 1;
                    }
                }
            }
            res + 1
        };
        let mut route = HashSet::new();
        Self::dfs(&grid, (m - 1, n - 1), cur, target, &mut route)
    }

    fn dfs(
        grid: &[Vec<i32>],
        bounds: (usize, usize),
        cur: (usize, usize),
        target: usize,
        route: &mut HashSet<(usize, usize)>,
    ) -> i32 {
        let (m, n) = bounds;
        let (i, j) = cur;

        if grid[i][j] == 2 && route.len() == target {
            return 1;
        }
        route.insert(cur);
        let mut try_dfs = |pos: (usize, usize)| {
            if grid[i][j] == -1 || route.contains(&pos) {
                return 0;
            };
            return Self::dfs(grid, bounds, pos, target, route);
        };
        let mut res = 0;
        if i > 0 {
            res += try_dfs((i - 1, j));
        }
        if i < m {
            res += try_dfs((i + 1, j));
        }
        if j > 0 {
            res += try_dfs((i, j - 1));
        }
        if j < n {
            res += try_dfs((i, j + 1));
        }
        route.remove(&cur);
        res
    }
}