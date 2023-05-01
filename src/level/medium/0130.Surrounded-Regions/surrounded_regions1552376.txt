// https://leetcode.com/problems/surrounded-regions/solutions/1552376/rust-dfs/
const MARKED: char = '!';
const FREE: char = 'O';
const CAPTURED: char = 'X';

pub fn solve(board: &mut Vec<Vec<char>>) {
    // mark cells connected to the border
    for col in 0..board[0].len() {
        if board[0][col] == FREE {
            mark_connected_cells(board, 0, col);
        }
    }
    for col in 0..board[board.len() - 1].len() {
        if board[board.len() - 1][col] == FREE {
            mark_connected_cells(board, board.len() - 1, col);
        }
    }
    for row in 1..board.len() - 1 {
        if board[row][0] == FREE {
            mark_connected_cells(board, row, 0);
        }

        if board[row][board[row].len() - 1] == FREE {
            mark_connected_cells(board, row, board[row].len() - 1);
        }
    }

    //mark the remaining FREE cells as CAPTURED and the MARKED cells as FREE
    for r in 0..board.len() {
        for c in 0..board[r].len() {
            if board[r][c] == MARKED {
                board[r][c] = FREE;
            } else if board[r][c] == FREE {
                board[r][c] = CAPTURED;
            }
        }
    }
}

fn mark_connected_cells(board: &mut Vec<Vec<char>>, r: usize, c: usize) {
    if board[r][c] != FREE {
        return;
    }

    board[r][c] = MARKED;

    if r > 0 {
        mark_connected_cells(board, r - 1, c);
    }
    if c > 0 {
        mark_connected_cells(board, r, c - 1);
    }
    if c < board[r].len() - 1 {
        mark_connected_cells(board, r, c + 1);
    }
    if r < board.len() - 1 {
        mark_connected_cells(board, r + 1, c);
    }
}