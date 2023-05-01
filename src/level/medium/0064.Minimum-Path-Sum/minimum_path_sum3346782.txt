// https://leetcode.com/problems/minimum-path-sum/solutions/3346782/rust-solution/
use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut mem_tab = vec![vec![0; grid[0].len()+1]; grid.len() + 1];
       for i in 1..mem_tab.len() {
           for j in 1..mem_tab[0].len(){
               match (i,j) {
                   (_,1) => mem_tab[i][j] = mem_tab[i-1][j] + grid[i-1][0],
                   (1,_) => mem_tab[i][j] = mem_tab[i][j-1] + grid[0][j-1],
                   (_,_) => mem_tab[i][j] = min(mem_tab[i-1][j] + grid[i-1][j-1], 
                                                mem_tab[i][j-1] + grid[i-1][j-1])
               }
           }
       }
        *mem_tab.last().unwrap().last().unwrap()
    }
}