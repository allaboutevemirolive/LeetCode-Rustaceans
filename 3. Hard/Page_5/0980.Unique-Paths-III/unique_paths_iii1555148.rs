// https://leetcode.com/problems/unique-paths-iii/solutions/1555148/rust-dfs-solution/
impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let mut count_walk: i32 = 0;
        let mut start: (usize, usize) = (0,0);
        
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    count_walk+=1;
                }
                if grid[i][j] == 1 {
                    start.0 = i;
                    start.1 = j;
                }
            }
        }
        
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        let mut ans: i32 = 0;
        
        fn solve(grid: &Vec<Vec<i32>>, pos: (usize, usize), count_walk: i32, count: i32,
            ans: &mut i32, visited: &mut Vec<Vec<bool>>) {
                
            visited[pos.0][pos.1] = true;
            
                
            if grid[pos.0][pos.1] == 2 && count-1 == count_walk {
                *ans+=1;
                visited[pos.0][pos.1] = false;
                return;
            }
                
            if pos.0 > 0 && !visited[pos.0 - 1][pos.1] && grid[pos.0-1][pos.1] != -1 {
                solve(grid, (pos.0-1, pos.1), count_walk, count+1, ans, visited);
            }
            
            if pos.0 < grid.len()-1 && !visited[pos.0 + 1][pos.1] && grid[pos.0+1][pos.1] != -1 {
                solve(grid, (pos.0+1, pos.1), count_walk, count+1, ans, visited);
            }
                
            if pos.1 > 0 && !visited[pos.0][pos.1-1] && grid[pos.0][pos.1-1] != -1 {
                solve(grid, (pos.0, pos.1-1), count_walk, count+1, ans, visited);
            }
                
            if pos.1 < grid[0].len()-1 && !visited[pos.0][pos.1+1] && grid[pos.0][pos.1+1] != -1 {
                solve(grid, (pos.0, pos.1+1), count_walk, count+1, ans, visited);
            }
                
            visited[pos.0][pos.1] = false;
        }
        
        solve(&grid, (start.0, start.1), count_walk, 0, &mut ans, &mut visited);
        
        ans
    }
}