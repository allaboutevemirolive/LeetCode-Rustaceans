// https://leetcode.com/problems/unique-paths-iii/solutions/2973538/rust-brute-force-dfs-solution-with-comments/
impl Solution {
    pub fn search(
        grid: &[Vec<i32>],
        start: (i32, i32),
        field_size: usize,
        wall_count: i32,
        paths: &mut i32,
        visited_count: i32,
        visited: &mut Vec<Vec<bool>>,
    ) {
        // check if we are at finish
        if grid[start.1 as usize][start.0 as usize] == 2 {
            // check if this path is valid (all walkable cells are painted)
            if visited_count == field_size as i32 - wall_count {
                *paths += 1;
            }
            return;
        }
        // neighbor cells
        let (up, right, down, left) = (
            (start.0, start.1 - 1),
            (start.0 + 1, start.1),
            (start.0, start.1 + 1),
            (start.0 - 1, start.1),
        );
        // walk in each direction
        for direction in [up, right, down, left] {
            // check if the next cell is in bounds of matrix
            if direction.0 < 0 || direction.0 as usize >= grid[0].len() {
                continue;
            }
            if direction.1 < 0 || direction.1 as usize >= grid.len() {
                continue;
            }
            let (x, y) = (direction.0 as usize, direction.1 as usize);
            // check if the next cell is already been visited;
            if visited[y][x] {
                continue;
            }
            // check if the next cell is impassable;
            if grid[y][x] == -1 {
                continue;
            }
            // mark the next cell as visited;
            visited[start.1 as usize][start.0 as usize] = true;
            // step into the next cell and do the remaining work
            Solution::search(
                grid,
                direction,
                field_size,
                wall_count,
                paths,
                visited_count + 1,
                visited,
            );
            // backtrack
            visited[start.1 as usize][start.0 as usize] = false;
        }
    }

    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        // dimensions
        let n = grid.len();
        let m = grid[0].len();
        let field_size = n * m;
        // find the starting cell and count the walkable cells
        let mut start = (0, 0);
        let mut wall_count = 0;
        for i in 0..n {
            for j in 0..m {
                match grid[i][j] {
                    1 => start = (j as i32, i as i32),
                    -1 => wall_count += 1,
                    _ => (),
                }
            }
        }

        let mut visited = vec![vec![false; m]; n];
        visited[start.1 as usize][start.0 as usize] = true;

        let visited_count = 1;
        let mut paths = 0;

        Solution::search(
            &grid,
            start,
            field_size,
            wall_count,
            &mut paths,
            visited_count,
            &mut visited,
        );
        paths
    }
}