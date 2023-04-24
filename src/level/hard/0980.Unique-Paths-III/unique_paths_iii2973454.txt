// https://leetcode.com/problems/unique-paths-iii/solutions/2973454/rust-elixir-backtracking-solution-extra/
impl Solution {
    const DIR: [usize; 5] = [0, 1, 0, usize::MAX, 0];

    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut zeros = 0;
        let mut x = 0;
        let mut y = 0;
        for (i, v) in grid.iter().enumerate() {
            for (j, &val) in v.iter().enumerate() {
                if val == 1 {
                    x = i;
                    y = j;
                }
                else if val == 0 {
                    zeros += 1;
                }
            }
        }
        Self::count_paths(&mut grid, x, y, zeros + 1)
    }

    fn count_paths(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, steps: i32) -> i32 {
        if x >= grid.len() || y >= grid[0].len() || grid[x][y] == -1 {
            return 0;
        }
        if grid[x][y] == 2 {
            return if steps == 0 { 1 } else { 0 };
        }
        let mut ans = 0;
        grid[x][y] = -1;
        for d in 0..4 {
            let x2 = x.wrapping_add(Self::DIR[d]);
            let y2 = y.wrapping_add(Self::DIR[d + 1]);
            ans += Self::count_paths(grid, x2, y2, steps - 1);
        }
        grid[x][y] = 0;
        ans
    }
}