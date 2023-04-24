// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/solutions/2758422/rust-bfs-solution-2ms-100/
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);
        let mut visited = vec![vec![None; cols as usize]; rows as usize];
        visited[0][0] = Some(k);
        let mut vd = VecDeque::new();
        vd.push_back(((0, 0), 0, k));
        let dir = [-1, 0, 1, 0, -1];
        while let Some(((i, j), steps, k)) = vd.pop_front() {
            if i == rows - 1 && j == cols - 1 {
                return steps;
            }
            for (&di, &dj) in dir.iter().zip(dir.iter().skip(1)) {
                let new_i = i + di as i32;
                let new_j = j + dj as i32;
                if new_i < 0 || new_i >= rows || new_j < 0 || new_j >= cols || k - grid[new_i as usize][new_j as usize] < 0 {
                    continue;
                }
                let (new_i, new_j) = (new_i as usize, new_j as usize);
                
                if let Some(v) = visited[new_i][new_j] {
                    if k - grid[new_i][new_j] <= v {
                        continue;
                    }
                }
                visited[new_i][new_j] = Some(k - grid[new_i][new_j]);
                vd.push_back(((new_i as i32, new_j as i32), steps + 1, k - grid[new_i][new_j]));
            }
        }
        -1
    }
}