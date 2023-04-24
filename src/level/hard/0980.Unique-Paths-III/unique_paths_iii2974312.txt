// https://leetcode.com/problems/unique-paths-iii/solutions/2974312/rust-dfs-backtracking-simple-traversal-is-enough/
use std::collections::HashSet;


impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        fn start(grid: &Vec<Vec<i32>>) -> (usize, usize) {
            grid.iter()
                .enumerate()
                .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, _)| (x, y)))
                .find(|(x, y)| grid[*x][*y] == 1)
                .or(Some((0, 0)))
                .unwrap()
        }

        fn get_neighbors(root: (usize, usize), grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
            let (x, y) = root;
            [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .filter_map(|(x, y)| {
                    if *x < grid.len() && *y < grid[0].len() {
                        Some((*x, *y))
                    } else {
                        None
                    }
                })
                .collect()
        }

        fn num_to_visit(grid: &Vec<Vec<i32>>) -> usize {
            grid.iter()
                .flat_map(|x| x.iter().filter(|e| **e >= 0))
                .collect::<Vec<&i32>>()
                .len()
        }

        fn dfs(root: (usize, usize), grid: &mut Vec<Vec<i32>>, num_to_visit: usize) -> i32 {
            let (x, y) = root;

            if grid[x][y] == 2 {
                if num_to_visit == 0 {
                    1
                } else {
                    0
                }
            } else if grid[x][y] < 0 {
                0
            } else {
                grid[x][y] = -3;
                let sum: i32 = get_neighbors(root, grid)
                    .into_iter()
                    .map(|n| dfs(n, grid, num_to_visit - 1))
                    .sum();
                grid[x][y] = 0;
                sum
            }
        }

        dfs(start(&grid), &mut grid.clone(), num_to_visit(&grid) - 1)
    }
}
