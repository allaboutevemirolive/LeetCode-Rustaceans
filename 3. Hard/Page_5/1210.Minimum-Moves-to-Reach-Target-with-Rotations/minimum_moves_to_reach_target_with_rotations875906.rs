// https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations/solutions/875906/rust-translated-8ms-100/
impl Solution {
    pub fn minimum_moves(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;

        fn can_rotate(g: &[Vec<i32>], r: i32, c: i32) -> bool {
            r < g.len() as i32 - 1
                && c < g[0].len() as i32 - 1
                && (g[r as usize + 1][c as usize] & 1) == 0
                && (g[r as usize][c as usize + 1] & 1) == 0
                && (g[r as usize + 1][c as usize + 1] & 1) == 0
        }

        fn can_go_down(g: &[Vec<i32>], r: i32, c: i32, vertical: bool) -> bool {
            if vertical {
                r < g.len() as i32 - 2 && (g[r as usize + 2][c as usize] & 1) == 0
            } else {
                r < g.len() as i32 - 1
                    && (g[r as usize + 1][c as usize] & 1) == 0
                    && (g[r as usize + 1][c as usize + 1] & 1) == 0
            }
        }

        fn can_go_right(g: &[Vec<i32>], r: i32, c: i32, vertical: bool) -> bool {
            if !vertical {
                c < g[0].len() as i32 - 2 && (g[r as usize][c as usize + 2] & 1) == 0
            } else {
                c < g[0].len() as i32 - 1
                    && (g[r as usize][c as usize + 1] & 1) == 0
                    && (g[r as usize + 1][c as usize + 1] & 1) == 0
            }
        }

        let mut steps = 0;
        let mut q = VecDeque::<(i32, i32, bool)>::new();
        q.push_back((0, 0, false));
        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let (r, c, vertical) = q.pop_front().unwrap();
                if r == grid.len() as i32 - 1 && c == grid[0].len() as i32 - 2 {
                    return steps;
                }
                if grid[r as usize][c as usize] & (if vertical { 2 } else { 4 }) == 0 {
                    grid[r as usize][c as usize] |= if vertical { 2 } else { 4 };
                    if can_go_down(&grid, r, c, vertical) {
                        q.push_back((r + 1, c, vertical));
                    }
                    if can_go_right(&grid, r, c, vertical) {
                        q.push_back((r, c + 1, vertical));
                    }
                    if can_rotate(&grid, r, c) {
                        q.push_back((r, c, !vertical));
                    }
                }
            }
            steps += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_moves() {
        assert_eq!(
            Solution::minimum_moves(vec![
                vec![0, 0, 0, 0, 0, 1],
                vec![1, 1, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1, 1],
                vec![0, 0, 1, 0, 1, 0],
                vec![0, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 0, 0, 0]
            ]),
            11
        );
    }

    #[test]
    fn test_minimum_moves_02() {
        assert_eq!(
            Solution::minimum_moves(vec![
                vec![0, 0, 1, 1, 1, 1],
                vec![0, 0, 0, 0, 1, 1],
                vec![1, 1, 0, 0, 0, 1],
                vec![1, 1, 1, 0, 0, 1],
                vec![1, 1, 1, 0, 0, 1],
                vec![1, 1, 1, 0, 0, 0]
            ]),
            9
        );
    }
}