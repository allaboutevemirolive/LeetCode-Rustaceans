// https://leetcode.com/problems/surrounded-regions/solutions/2220320/rust-dfs/
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        
        // mark top and bottom row
        for i in 0..m{
            for j in [0, n-1]{
                if board[i][j] == 'O'{
                    dfs_mark(board, i, j);
                }
            }
        }
        
        // mark left and right col
        for j in 0..n{
            for i in [0, m-1]{
                if board[i][j] == 'O'{
                    dfs_mark(board, i, j);
                }
            }
        }
        
        for i in 0..m{
            for j in 0..n{
                if board[i][j] == 'O'{
                    board[i][j] = 'X';
                }else if board[i][j] == '#'{
                    board[i][j] = 'O';
                }
            }
        }
        
    }
}

fn dfs_mark(board: &mut Vec<Vec<char>>, i: usize, j: usize){
    let (m, n) = (board.len(), board[0].len());
    
    // mark as visited
    board[i][j] = '#';
    
    for (i, j) in [(i+ 1, j), (i-1, j), (i, j+1), (i, j-1)]{
        if i!=usize::MAX && i < m && j!=usize::MAX && j < n && board[i][j] == 'O'{
            dfs_mark(board, i, j);
        }
    }
}