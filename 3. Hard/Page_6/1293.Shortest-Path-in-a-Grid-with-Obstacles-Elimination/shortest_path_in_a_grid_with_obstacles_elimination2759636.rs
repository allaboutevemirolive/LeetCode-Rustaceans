// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/solutions/2759636/rust-simple-bfs/
use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![-1; grid[0].len()]; grid.len()];
        
        queue.push_back((0, 0, 0, k));
        
        let directions: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        while let Some((x, y, len, mut remaining)) = queue.pop_front() {
            
            if x < 0 || x >= grid.len() || y < 0 || y >= grid[0].len() {
                continue;
            }
            
            if x == grid.len()-1 && y == grid[0].len()-1 {
                return len;
            }
            
            if grid[x][y] == 1 {
                if remaining > 0 {
                    remaining -= 1;
                } else {
                    continue;
                }
            }
            
            if(visited[x][y] != -1 && visited[x][y] >= remaining) {
                continue;    
            }
            
            visited[x][y] = remaining;
            
            for (dx, dy) in &directions {
                let (i, j) = (Self::add(x, *dx), Self::add(y, *dy));    
                queue.push_back((i, j, len+1, remaining));
            }
        }
        -1
    }
    
    fn add(u: usize, i: i32) -> usize {
        if i.is_negative() {
            u - i.wrapping_abs() as u32 as usize
        } else {
            u + i as usize
        }
    }
}