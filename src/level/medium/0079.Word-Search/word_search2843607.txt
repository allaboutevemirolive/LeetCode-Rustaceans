// https://leetcode.com/problems/word-search/solutions/2843607/rust-backtracking-solution/
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if board.is_empty() {
            return false;
        }
        let mut board = board;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::backtracking(&mut board, &word, i, j, 0) {
                    return true;
                }
            }
        }
        false
    }
    fn backtracking(
        board: &mut Vec<Vec<char>>, 
        word: &str, 
        i: usize, 
        j: usize, 
        idx: usize
    ) -> bool {
        if idx == word.len() {
            return true;
        }
        if i >= board.len() || j >= board[0].len() || word.chars().nth(idx) != Some(board[i][j]) {
            return false;
        }
        let ch = board[i][j];
        board[i][j] = '*';
        let mut found = Self::backtracking(board, word, i + 1, j, idx + 1);
        if !found && i > 0 {
            found = Self::backtracking(board, word, i - 1, j, idx + 1);
        }
        if !found {
            found = Self::backtracking(board, word, i, j + 1, idx + 1);
        }
        if !found && j > 0 {
            found = Self::backtracking(board, word, i, j - 1, idx + 1);
        }
        board[i][j] = ch;
        found
    }
}