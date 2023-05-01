// https://leetcode.com/problems/game-of-life/solutions/3249486/rust-1ms/
use std::collections::HashMap;
type Cell = (usize, usize);
impl Solution {
    fn get_original(board: &mut Vec<Vec<i32>>, originals: &mut HashMap<Cell, i32>, cell: &Cell) -> i32 {
        if let Some(initial) = originals.get(cell) {
            *initial
        } else {
            originals.insert(*cell, board[(*cell).0][(*cell).1]);
            board[(*cell).0][(*cell).1]
        }
    }
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let (m, n) = (board.len(), board[0].len());
        let directions: Vec<(i32, i32)> = vec![(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0),(1,-1),(0,-1)];
        let mut originals: HashMap<Cell, i32> = HashMap::new();
        let mut live_neighbors: i32 = 0;
        let mut original: i32 = 0;
        let mut cur_cell: Cell = (0, 0);
        for row in 0..m {
            for col in 0..n {
                live_neighbors = 0;
                cur_cell = (row, col);
                original = Self::get_original(board, &mut originals, &cur_cell);
                for (i, j) in directions.iter() {
                    if (row == 0 && *i == -1) || (row == m-1 && *i == 1)  {
                        continue;
                    } else if (col == 0 && *j == -1) || (col == n-1 && *j == 1) {
                        continue;
                    } else {
                        cur_cell = ((row as i32+*i) as usize, ((col as i32)+*j) as usize);
                        live_neighbors += Self::get_original(board, &mut originals, &cur_cell);
                    }
                }
                if original == 0 {
                    if live_neighbors == 3 {
                        board[row][col] = 1;
                    }
                } else {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        board[row][col] = 0;
                    }
                }
            }
        }
    }
}