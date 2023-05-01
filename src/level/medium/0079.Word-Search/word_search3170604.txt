// https://leetcode.com/problems/word-search/solutions/3170604/rust-dfs/

fn indexof(s: &String, i : usize) -> char {
    s.as_bytes()[i] as char
}
impl Solution {
    fn dfs(board: &Vec<Vec<char>>, word: &String, wordi: usize, i: usize, j: usize, visited : &mut Vec<Vec<bool>>) -> bool {
        if wordi == word.len()-1 {
            return true;
        }
        visited[i][j] = true;
        if ((i as i32)-1 >= 0) && (board[i-1][j] == indexof(word, wordi+1)) {
            if visited[i-1][j] == false{
                if Self::dfs(board, word, wordi+1, i-1, j, visited) {
                    return true;
                }
            }
        }
        if (i+1 < board.len()) && (board[i+1][j] == indexof(word, wordi+1)) {
            if visited[i+1][j] == false{
                if Self::dfs(board, word, wordi+1, i+1, j, visited) {
                    return true;
                }
            }
        }
        if ((j as i32)-1 >= 0) && (board[i][j-1] == indexof(word, wordi+1)) {
            if visited[i][j-1] == false{
                if Self::dfs(board, word, wordi+1, i, j-1, visited) {
                    return true;
                }
            }
        }
        if (j+1 < board[0].len()) && (board[i][j+1] == indexof(word, wordi+1)) {
            if visited[i][j+1] == false{
                if Self::dfs(board, word, wordi+1, i, j+1, visited) {
                    return true;
                }
            }
        }
        visited[i][j] = false;
        false
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let firstchar = word.as_bytes()[0] as char;
        let mut result = false;
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        board.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, c)| {
                if *c == firstchar {
                    if Solution::dfs(&board, &word, 0, i, j, &mut visited) {
                        result = true;
                        return;
                    }
                }
            });
        });
        result
    }
}
