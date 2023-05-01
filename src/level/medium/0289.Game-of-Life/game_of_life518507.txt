// https://leetcode.com/problems/game-of-life/solutions/518507/clear-rust-double-100/
 impl Solution {
    fn will_be_alive(i: usize, j: usize, board: &Vec<Vec<i32>>) -> bool {
        let mut adjacent = 0;
        let height = board.len();
        let width = board[0].len();

        for di in -1..=1 {
            let i = i as isize + di;
            if i < 0 || i >= height as isize {
                continue;
            }
            for dj in -1..=1 {
                let j = j as isize + dj;
                if j < 0 || j >= width as isize || (di == 0 && dj == 0) {
                    continue;
                }
                adjacent += board[i as usize][j as usize] & 1;
            }
        }
        let alive = board[i][j];
        (alive == 1 && (adjacent == 2 || adjacent == 3)) || (alive == 0 && adjacent == 3)
    }

    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let height = board.len();
        let width = board[0].len();

        for i in 0..height {
            for j in 0..width {
                if Solution::will_be_alive(i, j, board) {
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