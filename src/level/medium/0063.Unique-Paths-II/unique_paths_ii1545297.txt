// https://leetcode.com/problems/unique-paths-ii/solutions/1545297/rust-backtrack-dp/
use std::collections::HashMap;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let length = obstacle_grid.len() as usize; 
        let mut visited_results :HashMap<String, i32> = HashMap::with_capacity(length*length);
        return Self::quick_unique_paths_with_obstacles(&mut visited_results, &obstacle_grid, 0, 0);
    }
    
    fn quick_unique_paths_with_obstacles(visited_results :&mut HashMap<String, i32>, obstacle_grid: &Vec<Vec<i32>>, x :usize, y :usize) -> i32 {
        
        if x == obstacle_grid[0].len() - 1 && y == obstacle_grid.len() - 1 {
            let mut result = 1;
            if obstacle_grid[y][x] == 1 {
                result = 0;
            }
            visited_results.insert(Self::generateKey(x, y), result);
            return result;
        } else if x >= obstacle_grid[0].len() || y >= obstacle_grid.len() { 
            visited_results.insert(Self::generateKey(x, y), 0);
            return 0; 
        } else if obstacle_grid[y][x] == 1 { 
            visited_results.insert(Self::generateKey(x, y), 0);
            return 0; 
        }
        
        let mut result = 0; 
        let key = Self::generateKey(x, y);
        match visited_results.get(&key) {
            Some(val) => {
                result += val; 
            },
            None => {
                result += Self::quick_unique_paths_with_obstacles(visited_results, &obstacle_grid, x + 1, y);
                result += Self::quick_unique_paths_with_obstacles(visited_results, &obstacle_grid, x, y + 1);
            }
        }
        
        visited_results.insert(Self::generateKey(x, y), result);
        result
    }
    
    fn generateKey(x: usize, y: usize) -> String {
        format!("{}-{}", x, y)
    }
}