// https://leetcode.com/problems/game-of-life/solutions/1938096/rust-easy-to-understand/
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let (m, n) = (board.len(), board[0].len());
        let mut changes: Vec<(usize, usize)> = Vec::new();
        for i in 0..m {
            for j in 0..n {
                let neighbor_count = Solution::count_neighbors(board, i, j, m, n);
                let new_state = if neighbor_count == 3 {
                    1
                } else if neighbor_count == 2 {
                    board[i][j]
                } else {
                    0
                };
                if new_state != board[i][j] {
                    changes.push((i, j));
                }
            }
        }
        for (i, j) in changes {
            board[i][j] = if board[i][j] == 1 { 0 } else { 1 };
        }
    }

    fn count_neighbors(board: &[Vec<i32>], i: usize, j: usize, m: usize, n: usize) -> i32 {
        let mut neighbors = 0;
        if i > 0 {
            if board[i - 1][j] == 1 {
                neighbors += 1;
            }
            if j > 0 && board[i - 1][j - 1] == 1 {
                neighbors += 1;
            }
            if j < n - 1 && board[i - 1][j + 1] == 1 {
                neighbors += 1;
            }
        }
        if i < m - 1 {
            if board[i + 1][j] == 1 {
                neighbors += 1;
            }
            if j > 0 && board[i + 1][j - 1] == 1 {
                neighbors += 1;
            }
            if j < n - 1 && board[i + 1][j + 1] == 1 {
                neighbors += 1;
            }
        }
        if j > 0 && board[i][j - 1] == 1 {
            neighbors += 1;
        }
        if j < n - 1 && board[i][j + 1] == 1 {
            neighbors += 1;
        }
        neighbors
    }
}