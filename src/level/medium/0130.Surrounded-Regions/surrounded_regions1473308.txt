// https://leetcode.com/problems/surrounded-regions/solutions/1473308/rust-dfs-100-faster/
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        // check in row boundaries: top row and bottom row
        for i in 0..board[0].len() {
            Self::dfs(board, 0, i);
            Self::dfs(board, board.len() - 1, i);
        }

        // check in column boundaries: leftmost column and rightmost column
        for i in 0..board.len() {
            Self::dfs(board, i, 0);
            Self::dfs(board, i, board[0].len() - 1);
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                // mark all remaining 'O' cells as 'X': they aren't connected to any of the 4 edges
                if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }

                // flip all visited 'V' cells back to 'O'
                if board[i][j] == 'V' {
                    board[i][j] = 'O';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, row: usize, column: usize) {
        let beyond_boundary =
            row < 0 || row >= board.len() || column < 0 || column >= board[0].len();

        if beyond_boundary || board[row][column] != 'O' {
            return;
        }

        // mark current cell as visited
        board[row][column] = 'V';

        Self::dfs(board, row.wrapping_sub(1), column);
        Self::dfs(board, row + 1, column);
        Self::dfs(board, row, column.wrapping_sub(1));
        Self::dfs(board, row, column + 1);
    }
}