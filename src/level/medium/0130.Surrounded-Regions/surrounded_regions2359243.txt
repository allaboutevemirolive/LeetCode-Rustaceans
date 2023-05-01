// https://leetcode.com/problems/surrounded-regions/solutions/2359243/rust-bfs/
use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let cols = board[0].len();
        if rows < 3 || cols < 3 {
            return;
        }
        
        for j in 0..cols {
            if board[0][j] == 'O' {
                Self::BFS(0, j, board);
            }
            if board[rows-1][j] == 'O' {
                Self::BFS(rows-1, j, board);
            }
        }
        
        for i in 0..rows {
            if board[i][0] == 'O' {
                Self::BFS(i, 0, board);
            }
            if board[i][cols-1] == 'O' {
                Self::BFS(i, cols-1, board);
            }
        }
        
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                } 
                if board[i][j] == 'T' {
                    board[i][j] = 'O';
                }
            }
        }
    }
    
    fn BFS(i: usize, j: usize, board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let cols = board[0].len();
        let mut queue   = VecDeque::new();
        queue.push_back((i, j));
        
        let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        while let Some((i, j)) = queue.pop_front() {
            board[i][j] = 'T';
            
            for (di, dj) in directions {
                let (x, y) = (Self::add(i, di), Self::add(j, dj));
                if x >= 1 && y >= 1 && x < rows-1 && y < cols-1 && board[x][y] == 'O' {
                    queue.push_back((x, y));
                    board[x][y] = 'T';
                }                               
            }
        }   
    }
    
    fn add(u: usize, i: i32) -> usize {
        if i.is_negative() {
            u - i.wrapping_abs() as u32 as usize
        } else {
            u + i as usize
        }
    }
}
