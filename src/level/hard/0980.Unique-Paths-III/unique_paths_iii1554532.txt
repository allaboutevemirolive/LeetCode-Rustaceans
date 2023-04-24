// https://leetcode.com/problems/unique-paths-iii/solutions/1554532/rust-backtracking/
impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        unique_paths_iii(grid)
    }
}

pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut r = 0;
    let mut c = 0;
    let mut blocked = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 1 {
                r = row;
                c = col;
            } else if grid[row][col] == -1 {
                blocked += 1;
            }
        }
    }

    let free_cells = grid.len() * grid[0].len() - blocked;
    let mut paths = 0;

    dfs(&mut grid, &mut paths, r, c, free_cells - 1);

    paths
}

fn dfs(grid: &mut Vec<Vec<i32>>, paths: &mut i32, r: usize, c: usize, cells: usize) {
    let current = grid[r][c];
    if current == 2 {
        if cells == 0 {
            *paths += 1;
        }
        return;
    }

    grid[r][c] = -1;

    if r > 0 && grid[r - 1][c] != -1 {
        dfs(grid, paths, r - 1, c, cells - 1);
    }
    if c > 0 && grid[r][c - 1] != -1 {
        dfs(grid, paths, r, c - 1, cells - 1);
    }
    if c < grid[r].len() - 1 && grid[r][c + 1] != -1 {
        dfs(grid, paths, r, c + 1, cells - 1);
    }
    if r < grid.len() - 1 && grid[r + 1][c] != -1 {
        dfs(grid, paths, r + 1, c, cells - 1);
    }

    grid[r][c] = current;
}