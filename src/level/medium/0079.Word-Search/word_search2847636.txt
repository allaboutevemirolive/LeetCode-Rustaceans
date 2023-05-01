// https://leetcode.com/problems/word-search/solutions/2847636/rust-succinct/
impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::search(&mut board, &word.chars().collect(), (i as i32, j as i32), 0) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn search(
        board: &mut Vec<Vec<char>>,
        word: &Vec<char>,
        pos: (i32, i32),
        char_idx: usize,
    ) -> bool {
        let i = pos.0;
        let j = pos.1;
        if i < 0
            || j < 0
            || i as usize >= board.len()
            || j as usize >= board[0].len()
            // || board[i as usize][j as usize] != '0' <- no need to check this due the following character equality check
            || char_idx >= word.len()
            || board[i as usize][j as usize] != word[char_idx]
        {
            return false;
        } else if char_idx == word.len() - 1 {
            return true;
        }
        let c = board[i as usize][j as usize];
        board[i as usize][j as usize] = '0';
        let result = Self::search(board, word, (i - 1, j), char_idx + 1)
            || Self::search(board, word, (i, j + 1), char_idx + 1)
            || Self::search(board, word, (i + 1, j), char_idx + 1)
            || Self::search(board, word, (i, j - 1), char_idx + 1);
        board[i as usize][j as usize] = c;
        result
    }
}