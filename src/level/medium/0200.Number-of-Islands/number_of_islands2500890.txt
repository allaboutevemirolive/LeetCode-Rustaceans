// https://leetcode.com/problems/number-of-islands/solutions/2500890/rust-solution/
use std::collections::HashSet;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut visited = HashSet::new();
        let mut res = 0;
        let size = (grid.len() as i32, grid[0].len() as i32);

        for (row, i) in grid.iter().zip(0..) {
            for (_, j) in row.iter().zip(0..).filter(|(&c, _)| c == '1') {
                if visited.insert((i, j)) {
                    Solution::dfs_map(&grid, &mut visited, (i, j), size);
                    res += 1;
                }
            }
        }
        res
    }

    fn dfs_map(
        grid: &[Vec<char>],
        visited: &mut HashSet<(i32, i32)>,
        pos: (i32, i32),
        size: (i32, i32),
    ) {
        let mut stack = vec![pos];
        let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        while let Some(pos) = stack.pop() {
            for &(di, dj) in &dirs {
                let new_i = pos.0 + di;
                let new_j = pos.1 + dj;
                if new_i >= 0
                    && new_j >= 0
                    && new_i < size.0
                    && new_j < size.1
                    && grid[new_i as usize][new_j as usize] == '1'
                    && visited.insert((new_i, new_j))
                {
                    stack.push((new_i, new_j))
                }
            }
        }
    }
}