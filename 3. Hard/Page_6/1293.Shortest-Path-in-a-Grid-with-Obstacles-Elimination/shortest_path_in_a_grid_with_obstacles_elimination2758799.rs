// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/solutions/2758799/rust-double-buffered-floodfill-bfs-includes-test-cases/
use std::collections::VecDeque;

/// Tile parts: (y, x, k)
type Tile = (usize, usize, usize);

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let K = k as usize;
        let height = grid.len();
        let width = grid[0].len();
        if height == 1 && width == 1 {
            return 0;
        }
        if height == 1 {
            let obstacle_count: i32 = grid[0].iter().sum();
            return if obstacle_count <= k { (width-1) as i32 } else { -1 };
        }
        if width == 1 {
            let obstacle_count: i32 = grid.into_iter().map(|r| r[0]).sum();
            return if obstacle_count <= k { (height-1) as i32 } else { -1 };
        }
        let mut seen = vec![ vec![ vec![false; K+1]; width]; height ];
        // Both queues only contain legal moves.
        let mut future: VecDeque<Tile> = VecDeque::new();
        let mut present: VecDeque<Tile> = VecDeque::new();
        present.push_back( (0, 0, K) );
        let mut path_length = 0;
        // Works like level-order traversal of a tree.
        while !present.is_empty() {
            // println!("Step {path_length}: about to process {} choices", present.len());
            while let Some((y, x, rem)) = present.pop_front() {
                // left
                if x > 0 && (grid[y][x-1] == 0 || (grid[y][x-1] == 1 && rem > 0)) {
                    let new_cost = rem - (grid[y][x-1] as usize);
                    if !seen[y][x-1][new_cost] {
                        seen[y][x-1][new_cost] = true;
                        future.push_back( (y, x-1, new_cost) );
                    }
                }
                // up
                if y > 0 && (grid[y-1][x] == 0 || (grid[y-1][x] == 1 && rem > 0)) {
                    let new_cost = rem - (grid[y-1][x] as usize);
                    if !seen[y-1][x][new_cost] {
                        seen[y-1][x][new_cost] = true;
                        future.push_back( (y-1, x, new_cost) );
                    }
                }
                // right
                if x+1 < width && (grid[y][x+1] == 0 || (grid[y][x+1] == 1 && rem > 0)) {
                    if x+1 == width - 1 && y == height - 1 {
                        // The target tile is always empty, so `rem` is irrelevant.
                        return path_length + 1;  // We made it!
                    }
                    let new_cost = rem - (grid[y][x+1] as usize);
                    if !seen[y][x+1][new_cost] {
                        seen[y][x+1][new_cost] = true;
                        future.push_back( (y, x+1, new_cost) );
                    }
                }
                // down
                if y+1 < height && (grid[y+1][x] == 0 || (grid[y+1][x] == 1 && rem > 0)) {
                    if x == width - 1 && y+1 == height - 1 {
                        return path_length + 1;  // We made it!
                    }
                    let new_cost = rem - (grid[y+1][x] as usize);
                    if !seen[y+1][x][new_cost] {
                        seen[y+1][x][new_cost] = true;
                        future.push_back( (y+1, x, new_cost) );
                    }
                }
            }
            std::mem::swap(&mut present, &mut future);
            path_length += 1;
        }
        -1
    }
}