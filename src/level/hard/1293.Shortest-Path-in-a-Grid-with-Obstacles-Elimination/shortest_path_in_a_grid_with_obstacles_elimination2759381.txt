// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/solutions/2759381/rust-bfs-solution-with-comments/
use std::collections::VecDeque;
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut queue = VecDeque::<(i32, i32, i32, i32)>::from([(0, 0, 0, 0)]);
        let mut removed_prior = vec![vec![k; grid[0].len()]; grid.len()];
        
        while queue.len() > 0 {
            let (x, y, mut obstacles_removed, step_count) = queue.pop_front().unwrap();
            
            // check out of bounds
            if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
                continue;
            }
                        
            // check if soln
            if y as usize == grid.len() - 1 && x as usize == grid[0].len() - 1 {
                return step_count;
            }
            
            // if obstacle, try to remove it
            if grid[y as usize][x as usize] == 1 {
                obstacles_removed += 1;
                if obstacles_removed > k {
                    continue;
                }
            }
            
            // check if someone has been here
            // and if their path was more optimal
            // (removal wise) then we can just quit
            if obstacles_removed > removed_prior[y as usize][x as usize] {
                continue;
            } else {
                removed_prior[y as usize][x as usize] = obstacles_removed - 1;
            }
            
            queue.push_back((x + 1, y, obstacles_removed, step_count + 1));
            queue.push_back((x - 1, y, obstacles_removed, step_count + 1));
            queue.push_back((x, y + 1, obstacles_removed, step_count + 1));
            queue.push_back((x, y - 1, obstacles_removed, step_count + 1));
        }
        
        -1
    }
}