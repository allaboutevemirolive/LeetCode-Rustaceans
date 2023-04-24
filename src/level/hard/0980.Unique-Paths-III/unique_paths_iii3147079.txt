// https://leetcode.com/problems/unique-paths-iii/solutions/3147079/my-rust-solution/
use std::collections::HashSet;
impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut is_visited = HashSet::new();
        let mut start:(i32, i32) = (0, 0);
        let mut obstacles_counter = 0;
        for (i, ves) in grid.iter().enumerate() {
            for (j, &val) in ves.iter().enumerate() {
                if val == 1 {
                    start = (i as i32, j as i32);
                    is_visited.insert((i as i32, j as i32));
                }
                if val == -1 {
                    obstacles_counter += 1;
                }
            }
        }
        Self::search(HashSet::new(), &grid, (start.0 + 1, start.1), obstacles_counter) +
            Self::search(HashSet::new(), &grid, (start.0 - 1, start.1), obstacles_counter) +
            Self::search(HashSet::new(), &grid, (start.0, start.1 + 1), obstacles_counter) +
            Self::search(HashSet::new(), &grid, (start.0, start.1 - 1), obstacles_counter)
    }

    pub fn search(mut is_visited: HashSet<(i32, i32)>, grid: &Vec<Vec<i32>>, (i, j): (i32, i32), obstacles_counter: i32) -> i32 {
        if i < 0 || i == grid.len() as i32 || j < 0 || j == grid[0].len() as i32 {
            return 0
        }
        if is_visited.contains(&(i, j)) {
            return 0
        }
        match grid[i as usize][j as usize] {
            2 => {
                if is_visited.len() == grid.len() * grid[0].len() - obstacles_counter as usize - 2 {
                    return 1;
                }
                0
            }
            -1 => 0,
            0 => {
                is_visited.insert((i, j));
                Self::search(is_visited.clone(), grid, (i+1,j), obstacles_counter) +
                    Self::search(is_visited.clone(), grid, (i-1,j), obstacles_counter) +
                    Self::search(is_visited.clone(), grid, (i,j+1), obstacles_counter) +
                    Self::search(is_visited.clone(), grid, (i,j-1), obstacles_counter)
            }
            _ => 0
        }
    }
}