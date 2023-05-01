// https://leetcode.com/problems/surrounded-regions/solutions/3384134/rust-intuitive-solution/
impl Solution {
    fn flip(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if board[i][j] != 'O' {
            return;
        }
        
        let it: Vec<i32> = vec![-1, 1];

        board[i][j] = 'M';

        for displ in it.iter() {
            let k = i as i32 + displ;
            let l = j as i32 + displ;
            if k >= 0 && k < board.len() as i32
            && l >= 0 && l < board[0].len() as i32 {
                Self::flip(board, k as usize, j);
                Self::flip(board, i, l as usize);
            }
        }
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        // An 'O' should not be flipped if:
        // - It is on the border, or
        // - It is adjacent to an 'O' that should not be flipped.

        if board.len() <= 2 || board[0].len() <= 2 {
            return;
        }

        let rows = board.len();
        let cols = board[0].len();

        // iterate over edges;
        // if there's an 'O': flip all adjusted
        // 'O's to, let's say, 'M's. Then iterate
        // through array. Flip all 'M's to 'O's, 
        // and all 'O's to 'X's

        // set corners to 'M's if they're 'O's
        if board[0][0] == 'O' {
            board[0][0] = 'M';
        }
        if board[rows-1][0] == 'O' {
            board[rows-1][0] = 'M';
        }
        if board[0][cols-1] == 'O' {
            board[0][cols-1] = 'M';
        }
        if board[rows-1][cols-1] == 'O' {
            board[rows-1][cols-1] = 'M';
        }


        // 1st row/column; 
        let first = 0;
        // last row; 
        let last = rows - 1;

        for j in 1..(cols - 1) {
            Self::flip(board, first, j);
            Self::flip(board, last, j);
        }

        // last column
        let last = cols - 1;

        for i in 1..(rows - 1) {
            Self::flip(board, i, first);
            Self::flip(board, i, last);
        }

        // last iteration: change 'M's to 'O's
        // and 'O's to 'X's
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'M' {
                    board[i][j] = 'O';
                } else if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }
    }
}