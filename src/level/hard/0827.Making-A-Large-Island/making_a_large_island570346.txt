// https://leetcode.com/problems/making-a-large-island/solutions/570346/rust-naive-bfs/
use std::collections::VecDeque;

fn max_island(mut g: &mut Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
    let mut res = 0;
    let mut visited = vec![vec![false; g[0].len()]; g.len()];
    if x < g.len() && y < g[0].len() {
        g[x][y] = 1; 
    }
    for i in 0..g.len() {
        for j in 0..g[0].len() {
            if g[i][j] == 0 { continue; }
            if visited[i][j] { continue; }
            
            res = res.max(island_size(&g, i, j, &mut visited));
        }
    }
    if x < g.len() && y < g[0].len() {
        g[x][y] = 0; 
    }
    res
}

fn island_size(grid: &Vec<Vec<i32>>, x: usize, y: usize, mut visited: &mut Vec<Vec<bool>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    
    let mut q = VecDeque::new();
    q.push_back((x, y));
    visited[x][y] = true;
    
    let mut res = 1;
    while let Some((i, j)) = q.pop_front() {
        let nexts = vec![
            (i+1, j),
            (i, j+1),
            (i-1, j),
            (i, j-1),
        ];
        
        for (next_i, next_j) in nexts.into_iter() {
            if next_i > m - 1 { continue; }
            if next_j > n - 1 { continue; }
            
            if grid[next_i][next_j] == 0 { continue; }
            if visited[next_i][next_j] { continue; }
            
            q.push_back((next_i, next_j));
            res += 1;
            visited[next_i][next_j] = true;
        }
    }
    res
}

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut g: Vec<Vec<i32>> = grid.iter()
            .map(|r| r.iter().map(|x| *x).collect())
            .collect();
        
        let mut res = max_island(&mut g, 100, 100);
        
        for i in 0..g.len() {
            for j in 0..g[0].len() {
                if g[i][j] == 1 { continue; }
                
                res = res.max(max_island(&mut g, i, j));
            }
        }
        res
    }
}