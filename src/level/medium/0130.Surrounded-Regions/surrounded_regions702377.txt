// https://leetcode.com/problems/surrounded-regions/solutions/702377/rust-dfs-12ms/
use std::collections::VecDeque;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() <=2 { return; }

        let mut q = VecDeque::new();
        let (w, h) = (board[0].len() - 1, board.len() - 1);
        for ic in 0..=w {
            if board[0][ic] == 'O' { q.push_back((0, ic)); }
            if board[h][ic] == 'O' { q.push_back((h, ic)); }
        }
        for ir in 1..=h - 1 {
            if board[ir][0] == 'O' { q.push_back((ir, 0)); }
            if board[ir][w] == 'O' { q.push_back((ir, w)); }
        }
        while let Some((r, c)) = q.pop_front() {
            if board[r][c] != 'O' { continue; }
            if board[r][c] == 'O' {
                board[r][c] = '-';
                if r > 0 { q.push_back((r - 1, c)); }
                if r < h { q.push_back((r + 1, c)); }
                if c > 0 { q.push_back((r, c - 1)); }
                if c < w { q.push_back((r, c + 1)); }
            }
        }

        for ir in 0..=h {
            for ic in 0..=w {
                if board[ir][ic] == 'O' { board[ir][ic] = 'X'; } 
                else if board[ir][ic] == '-' { board[ir][ic] = 'O';}
            }
        }
    }
}