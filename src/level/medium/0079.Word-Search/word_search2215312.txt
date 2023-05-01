// https://leetcode.com/problems/word-search/solutions/2215312/rust-dfs/
impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        
        let (m, n) = (board.len(), board[0].len());
        let word = word.chars().collect::<Vec<char>>();
        
        for i in 0..m{
            for j in 0..n{
                if dfs(&mut board, &word, i, j){
                    return true;
                }
            }
        }
        
        false
    }
}

pub fn dfs(board: &mut Vec<Vec<char>>, word: &[char], i: usize, j: usize) -> bool{
    let (m, n) = (board.len(), board[0].len());
    
    if word.len() == 0{
        return true;
    }
    
    if i != usize::MAX && i < m && j != usize::MAX && j < n{
       
        let mut res = false;
        if board[i][j] != word[0]{
            return false;
        }
        
        // mark cell as temporarily as visited
        let tmp = board[i][j];
        board[i][j] = '#';
        
        let exists = [
            (i - 1, j), 
            (i + 1, j), 
            (i, j + 1), 
            (i, j - 1)
        ]
        .iter()
        .any(|p|{
            dfs(board, &word[1..], p.0, p.1)
        });
        
        // replace cell value after dfs
        board[i][j] = tmp;
        
        return exists;
        
    }else{
        false
    }
}