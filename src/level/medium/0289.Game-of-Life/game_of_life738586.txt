// https://leetcode.com/problems/game-of-life/solutions/738586/rust-cheapest-best/
use std::mem;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let height = board.len() as i32;
        let width = board[0].len() as i32;
        let next_state: Vec<Vec<i32>> = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| {
                        let n = [
                            (-1, -1),
                            (0, -1),
                            (1, -1),
                            (-1, 0),
                            (1, 0),
                            (-1, 1),
                            (0, 1),
                            (1, 1),
                        ]
                        .iter()
                        .map(|(add_x, add_y)| (x + add_x, y + add_y))
                        .filter(|(x, y)| *y >= 0 && *y < height && *x >= 0 && *x < width)
                        .filter(|(x, y)| board[*y as usize][*x as usize] == 1)
                        .count();

                        match board[y as usize][x as usize] {
                            0 => match n {
                                3 => 1,
                                _ => 0,
                            },
                            _ => match n {
                                0 | 1 => 0,
                                2 | 3 => 1,
                                _ => 0,
                            },
                        }
                    })
                    .collect::<Vec<i32>>()
            })
            .collect();
        mem::replace(board, next_state);
    }
}