// https://leetcode.com/problems/game-of-life/solutions/429213/0ms-rust-solution/
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        
        fn will_live(coord: (usize, usize), mat: &Vec<Vec<i32>>) -> bool {
            let num_alive_nei = live_neighbors(coord, mat);
            if num_alive_nei < 2 || num_alive_nei > 3 {
                return false;
            }
            if (num_alive_nei == 2 || num_alive_nei == 3) && mat[coord.0][coord.1] == 1 {
                return true;
            }
            if num_alive_nei == 3 && mat[coord.0][coord.1] == 0 {
                return true;
            }
            false
        }
        
        fn live_neighbors(coord: (usize, usize), mat: &Vec<Vec<i32>>) -> i32 {
            if mat.is_empty() {
                return 0;
            }
            let mut ret = 0;
            let height = mat.len();
            let width = mat[0].len();
            
            // can use for loops below to make it looks nicer...
            if coord.0+1 >= 0 && coord.0+1 < height && coord.1+1 >= 0 && coord.1+1 < width {
                if mat[coord.0+1][coord.1+1] & 1 == 1 {
                    ret += 1;
                }
            }
            if coord.0+1 >= 0 && coord.0+1 < height && coord.1-1 >= 0 && coord.1-1 < width {
                if mat[coord.0+1][coord.1-1] & 1 == 1 {
                    ret += 1;
                }
            }
            if coord.0-1 >= 0 && coord.0-1 < height && coord.1+1 >= 0 && coord.1+1 < width {
                if mat[coord.0-1][coord.1+1] & 1 == 1 {
                    ret += 1;
                }
            }
            if coord.0-1 >= 0 && coord.0-1 < height && coord.1-1 >= 0 && coord.1-1 < width {
                if mat[coord.0-1][coord.1-1] & 1 == 1 {
                    ret += 1;
                }
            }
            if coord.0+1 >= 0 && coord.0+1 < height && coord.1 >= 0 && coord.1 < width {
                if mat[coord.0+1][coord.1] & 1 == 1 {
                    ret += 1;
                }
            }
            if coord.0-1 >= 0 && coord.0-1 < height && coord.1 >= 0 && coord.1 < width {
                if mat[coord.0-1][coord.1] & 1 == 1 {
                    ret += 1;
                }
            }
            if coord.0 >= 0 && coord.0 < height && coord.1+1 >= 0 && coord.1+1 < width {
                if mat[coord.0][coord.1+1] & 1 == 1 {
                    ret += 1;
                }
            }
            if coord.0 >= 0 && coord.0 < height && coord.1-1 >= 0 && coord.1-1 < width {
                if mat[coord.0][coord.1-1] & 1 == 1 {
                    ret += 1;
                }
            }
            
            ret
        }
        
        if board.is_empty() {
            return
        }
        
        let height = board.len();
        let width = board[0].len();
        
        for i in 0..height {
            for j in 0..width {
                if will_live((i,j), board) {
                    board[i][j] |= 2;
                }
            }
        }
        
        for i in 0..height {
            for j in 0..width {
                board[i][j] >>= 1;
            }
        }
    }
}