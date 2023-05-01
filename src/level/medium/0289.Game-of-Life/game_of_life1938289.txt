// https://leetcode.com/problems/game-of-life/solutions/1938289/short-clean-100-in-place-with-rust-iter-mut-for-each/
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                let mut count = 0;
                for i1 in std::cmp::max(i as i32 - 1, 0) as usize ..= i + 1 {
                    for j1 in std::cmp::max(j as i32 - 1, 0) as usize ..= j + 1 {
                        if i1 < board.len() && j1 < board[0].len() && (i1 != i || j1 != j) && (board[i1][j1] + 2) % 2 == 1 {
                            count += 1;
                        }
                    }
                }
                if count < 2 || count > 3{
                    board[i][j] -= 2;
                } else if count == 3 {
                    board[i][j] += 2;
                }
            }
        }
        board.iter_mut().for_each(|row| { row.iter_mut().for_each(|x| if *x > 0 { *x = 1 } else { *x = 0 }) });
    }
}