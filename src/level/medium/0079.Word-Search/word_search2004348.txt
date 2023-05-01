// https://leetcode.com/problems/word-search/solutions/2004348/rust-backtracking/
use std::collections::HashSet;

fn backtrack(board: &Vec<Vec<char>>, start: usize, row: usize, col: usize, word: &Vec<char>, used: &mut HashSet<(usize, usize)>) -> bool {
  let m = board.len();
  let n = board[0].len();

  if start == word.len() {
    return true;
  } else if row >= m || col >= n || used.contains(&(row, col)) || board[row][col] != word[start] {
    return false;
  }
  used.insert((row,col));

  if backtrack(board, start+1, row+1, col, word, used) || backtrack(board, start+1, row, col+1, word, used) {
    return true;
  }
  if row > 0  && backtrack(board, start+1, row-1, col, word, used) {
    return true;
  }
  if col > 0 && backtrack(board, start+1, row, col-1, word, used) {
    return true;
  }
  used.remove(&(row,col));
  false
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let word: Vec<char> = word.chars().collect();
    let mut used = HashSet::with_capacity(word.len());
    let m = board.len();
    let n = board[0].len();
    for i in 0..m {
      for j in 0..n {
        if backtrack(&board, 0, i, j, &word, &mut used) {
          return true;
        }
      }
    }
    false
}