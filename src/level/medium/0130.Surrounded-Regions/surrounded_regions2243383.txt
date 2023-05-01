// https://leetcode.com/problems/surrounded-regions/solutions/2243383/rust-border-bfs/
use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Point {
    row: i32,
    col: i32
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut border = VecDeque::new();
        let mut visited: HashSet<Point> = HashSet::new();
        
        let num_rows = board.len();
        let num_cols = board[0].len();
                
        for i in 0 .. num_rows {
            if board[i][0] == 'O' {
                border.push_back(Point{row: i as i32, col: 0});
            }
            if board[i][num_cols - 1] == 'O' {
                border.push_back(Point{row: i as i32, col: (num_cols - 1usize) as i32});
            }
        }
        
        for i in 0 .. num_cols {
            if board[0][i] == 'O' {
                border.push_back(Point{row: 0, col: i as i32});
            }
            if board[num_rows - 1][i] == 'O' {
                border.push_back(Point{row: (num_rows - 1usize) as i32, col: i as i32});
            }
        }
        
        
        let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        while border.len() != 0 {
            let point = border.pop_front();
            if let Some(point) = point {
                if !visited.contains(&point) {
                    visited.insert(point.clone());
                    
                    let row = point.row;
                    let col = point.col;
                    
                    for offset in &offsets {
                        let new_row = row - offset.0;
                        let new_col = col - offset.1;
                        
                        if new_row >= 0 && new_col >= 0 && new_row < num_rows as i32 && new_col < num_cols as i32 {
                            if board[new_row as usize][new_col as usize] == 'O' {
                                border.push_back(Point{row: new_row, col: new_col});
                            }
                        }
                    }
                }
            }
        }
        
        for i in 0 .. num_rows {
            for j in 0 .. num_cols {
                if !visited.contains(&Point{row: i as i32, col: j as i32}) {
                    board[i][j] = 'X';
                }
            }
        }
    }
}