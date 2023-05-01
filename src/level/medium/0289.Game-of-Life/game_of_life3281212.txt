// https://leetcode.com/problems/game-of-life/solutions/3281212/rust-decent-solution/
use std::collections::HashMap;

impl Solution {
    fn checked_cell_getter(
        y: Option<usize>,
        x: Option<usize>,
        board: &mut Vec<Vec<i32>>,
    ) -> Option<i32> {
        if let Some(y) = y {
            if let Some(x) = x {
                if let Some(row) = board.get(y) {
                    if let Some(cell) = row.get(x) {
                        return Some(*cell);
                    }
                }
            }
        }
        None
    }

    fn get_neighbors(y: usize, x: usize, board: &mut Vec<Vec<i32>>) -> Vec<i32> {
        let neighbor_1 = Self::checked_cell_getter(y.checked_sub(1), x.checked_sub(1), board);
        let neighbor_2 = Self::checked_cell_getter(y.checked_sub(1), Some(x), board);
        let neighbor_3 = Self::checked_cell_getter(y.checked_sub(1), x.checked_add(1), board);
        let neighbor_4 = Self::checked_cell_getter(Some(y), x.checked_add(1), board);
        let neighbor_5 = Self::checked_cell_getter(y.checked_add(1), x.checked_add(1), board);
        let neighbor_6 = Self::checked_cell_getter(y.checked_add(1), Some(x), board);
        let neighbor_7 = Self::checked_cell_getter(y.checked_add(1), x.checked_sub(1), board);
        let neighbor_8 = Self::checked_cell_getter(Some(y), x.checked_sub(1), board);
        vec![
            neighbor_1, neighbor_2, neighbor_3, neighbor_4, neighbor_5, neighbor_6, neighbor_7,
            neighbor_8,
        ]
        .into_iter()
        .filter_map(|c| c)
        .collect()
    }

    fn apply_rules(cell: i32, neighbors: &[i32]) -> i32 {
        let alive_neighbors = neighbors.iter().filter(|n| **n == 1).collect::<Vec<_>>();
        if cell == 1 {
            // Any live cell with fewer than two live neighbors dies as if caused by under-population.
            if alive_neighbors.len() < 2 {
                return 0;
            }
            // Any live cell with two or three live neighbors lives on to the next generation.
            if alive_neighbors.len() == 2 || alive_neighbors.len() == 3 {
                return 1;
            }
            // Any live cell with more than three live neighbors dies, as if by over-population.
            if alive_neighbors.len() > 3 {
                return 0;
            }
        } else if cell == 0 {
            // Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
            if alive_neighbors.len() == 3 {
                return 1;
            }
        }
        return cell;
    }

    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let mut neighbors = HashMap::new();
        for y in 0..board.len() {
            for x in 0..board[y].len() {
                neighbors.insert((y, x), Self::get_neighbors(y, x, board));
            }
        }
        let mut new_board = (0..board.len())
            .map(|_| (0..board[0].len()).map(|_| 0).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for ((y, x), neighbors) in neighbors {
            let cell = board[y][x];
            let next_state = Self::apply_rules(cell, &neighbors);
            new_board[y][x] = next_state;
        }
        *board = new_board;
    }
}