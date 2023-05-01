// https://leetcode.com/problems/valid-sudoku/solutions/3199607/rust-three-1d-arrays/
impl Solution {
    const EMPTY: char = '.';
    const OFFSET: usize = '0' as usize;

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = board.len();

        for row in 0..n {
            let mut row_values = [false; 9];
            let mut col_values = [false; 9];
            let mut box_values = [false; 9];

            for col in 0..n {
                let box_value = board[(row/3) * 3 + (col/3)][(row%3) * 3 + (col%3)];
                
                if Solution::is_repeating(&mut row_values, board[row][col])
                    || Solution::is_repeating(&mut col_values, board[col][row])
                    || Solution::is_repeating(&mut box_values, box_value)
                {
                    return false;
                }
            }
        }
        true
    }

    fn is_repeating(list: &mut [bool], value: char) -> bool {
        if value != Solution::EMPTY {
            let value = value as usize - Solution::OFFSET;
            if list[value - 1] {
                return true;
            }
            list[value - 1] = true;
        }
        false
    }
}