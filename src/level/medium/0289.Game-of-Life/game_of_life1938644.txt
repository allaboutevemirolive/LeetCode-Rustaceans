// https://leetcode.com/problems/game-of-life/solutions/1938644/rust-solution-double-100/
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        fn next_state(board: &mut Vec<Vec<i32>>) {
            let height = board.len();
            let width = board[0].len();
            for i in 0..height {
                for j in 0..width {
                    let neighbor = calculate_alive_neighbors(
                        (i as i32,j as i32),
                        (height as i32, width as i32), 
                        board);
                    match (neighbor, board[i][j]) {
                        (3,_) => {
                            board[i][j] |= 2;
                        }
                        (2, 1) => {
                            board[i][j] |= 2;
                        }
                        (_,_) => {
                        }
                    }
                }
            }
            
            for i in 0..height {
                for j in 0..width {
                    board[i][j] >>= 1;
                }
            }
        }
        
        fn calculate_alive_neighbors
        (
            (x,y): (i32, i32),
            (height, width): (i32,i32), 
            board:&Vec<Vec<i32>>
        ) -> i32 {
            let mut count = 0;
            
            for i in (-1..=1 as i32) {
                for j in (-1..=1 as i32) {
                    match x+i >= 0 && x+i < height && y+j >= 0 && y+j < width {
                        true => {
                            match board[(x+i) as usize][(y+j) as usize] & 1 {
                                1 => count += 1,
                                _ => (),
                            }
                        }
                        false => (),
                    }
                }
            }
            count - board[x as usize][y as usize]
        }
        if board.len() > 0 {
            next_state(board); 
        }
        
    }
}