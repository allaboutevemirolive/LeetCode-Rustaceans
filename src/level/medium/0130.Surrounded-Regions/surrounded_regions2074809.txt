// https://leetcode.com/problems/surrounded-regions/solutions/2074809/rust-15ms-4-7mb/
impl Solution {
    pub fn solve(mut board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        let (mut row, mut col) = (board.len(), board[0].len());
        
        /*
            Scan for surrounded Regions (O -> X)
        */
        for r in 1..row - 1 { 
            Self::scan_board(board, r, 0);
            Self::scan_board(board, r, col - 1);

        }
        /*
            Scan border region
        */
        for c in 0..col { 
            Self::scan_board(board, 0, c);
            Self::scan_board(board, row - 1, c);
        }
        
        /*
            Uncapture unsurrounded regions (T -> O)
        */
        for k in 0..row * col { 
            board[k / col][k % col] = if board[k / col][k % col] == 'T' {
                'O'
            } else {
                'X'
            }
        }
        
    }
    /*
        Convert Any 'O' to 'T'
    */
    fn scan_board(mut board: &mut Vec<Vec<char>>, r: usize, c: usize) { 
        if let Some(coord) = board.get_mut(r).and_then(|f| f.get_mut(c)) { 
            if *coord == 'O' { 
                *coord = 'T';
                Self::scan_board(board, r - 1, c);
                Self::scan_board(board, r + 1, c);
                Self::scan_board(board, r, c - 1);
                Self::scan_board(board, r, c + 1);
            }
        }
    }
}