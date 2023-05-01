// https://leetcode.com/problems/number-of-islands/solutions/3349858/rust/
impl Solution {
    fn scan_island(grid: &Vec<Vec<char>>, used: &mut Vec<Vec<bool>>, start: (usize, usize), row: usize, col: usize) {
        let (i, j) = start;
        if i < 0 || j < 0 || i >= row || j >= col {
            return
        }
        if !used[i][j] && grid[i][j] != '0' {
            used[i][j] = true;
            let (i_l, i_r, j_u, j_d) = (i-1, i+1, j-1, j+1);
            // west
            Self::scan_island(grid, used, (i_l, j), row, col);
            // north
            Self::scan_island(grid, used, (i, j_u), row, col);
            // east
            Self::scan_island(grid, used, (i_r, j), row, col);
            // south
            Self::scan_island(grid, used, (i, j_d), row, col);
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (row, col) = (grid.len(), grid[0].len());
        let mut res = 0;
        let mut used: Vec<Vec<bool>> = vec![vec![false; col]; row];
        for i in 0..row {
            for j in 0..col {
                if !used[i][j] && grid[i][j] != '0' {
                    Self::scan_island(&grid, &mut used, (i, j), row, col);
                    res += 1;
                }
            }
        }
        res
    }
}