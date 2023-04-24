// https://leetcode.com/problems/unique-paths-iii/solutions/2975623/rust-dfs-solution/
impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut num_of_empty = 0;
        let mut start = (0, 0);
        for (i, col) in grid.iter().enumerate() {
            for (j, &s) in col.iter().enumerate() {
                match s {
                    0 => num_of_empty += 1,
                    1 => start = (i as i32, j as i32),
                    _ => (),
                }
            }
        }
        fn dfs(
            i: i32,
            j: i32,
            m: i32,
            n: i32,
            rest: i32,
            grid: &mut Vec<Vec<i32>>,
            result: &mut i32,
        ) {
            for (x, y) in vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                .into_iter()
                .filter(|&(x, y)| x >= 0 && x < m && y >= 0 && y < n)
            {
                let x = x as usize;
                let y = y as usize;
                if grid[x][y] == 0 {
                    grid[x][y] = -1;
                    dfs(x as i32, y as i32, m, n, rest - 1, grid, result);
                    grid[x][y] = 0;
                } else if grid[x][y] == 2 {
                    if rest == 0 {
                        *result += 1;
                    }
                }
            }
        }
        let mut result = 0;
        dfs(
            start.0,
            start.1,
            m as i32,
            n as i32,
            num_of_empty,
            &mut grid,
            &mut result,
        );
        result
    }
}