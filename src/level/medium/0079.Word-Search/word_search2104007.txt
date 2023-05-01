// https://leetcode.com/problems/word-search/solutions/2104007/rust-90ms-2-1mb/
impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let (mut row, mut col) = (board.len(), board[0].len());
        if word.len() > row * col { return false }
        for r in 0..row { 
            for c in 0..col { 
                if Self::word_search(&mut board, &word, r, c, 0) { 
                    return true 
                }
            }
        }
        false
    }
    fn word_search(mut board: &mut Vec<Vec<char>>, word: &String, row: usize, col: usize, index: usize) -> bool { 
        if index == word.len() { return true }
        if let Some(mut word_comb) = board.get_mut(row).and_then(|w| w.get_mut(col)) { 
            if *word_comb == word.as_bytes()[index] as char {
                let prev = board[row][col];
                board[row][col] = '-';

                let found = (
                Self::word_search(board, word, row - 1, col, index + 1) ||
                Self::word_search(board, word, row + 1, col, index + 1) ||
                Self::word_search(board, word, row, col - 1, index + 1) ||
                Self::word_search(board, word, row, col + 1, index + 1));
                board[row][col] = prev;
                
                return found 
            } else { 
                false
            }
        } else {
            false
        }
    }
}