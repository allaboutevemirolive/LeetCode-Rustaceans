// https://leetcode.com/problems/minimum-moves-to-move-a-box-to-their-target-location/solutions/876517/rust-translated-4ms-100/
impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        use std::collections::{HashSet, VecDeque};

        const DIRS: [i32; 5] = [0, 1, 0, -1, 0];

        fn can_reach_pos(
            grid: &[Vec<char>],
            x1: i32,
            y1: i32,
            x2: i32,
            y2: i32,
            bx: i32,
            by: i32,
            m: i32,
            n: i32,
        ) -> bool {
            let mut queue = VecDeque::<(i32, i32, i32, i32)>::new();
            queue.push_back((x1, y1, x2, y2));

            let mut set = HashSet::<i32>::new();

            set.insert(x1 * n + y1);
            set.insert(bx * n + by);

            while let Some(state) = queue.pop_front() {
                if state.0 == x2 && state.1 == y2 {
                    return true;
                }
                for i in 0..4 {
                    let px = state.0 + DIRS[i];
                    let py = state.1 + DIRS[i + 1];

                    if px < 0 || px > m - 1 || py < 0 || py > n - 1 {
                        continue;
                    }

                    if grid[px as usize][py as usize] != '#' && set.insert(px * n + py) {
                        queue.push_back((px, py, x2, y2));
                    }
                }
            }
            false
        }

        let mut bx = 0;
        let mut by = 0; // box
        let mut px = 0;
        let mut py = 0; // player
        let mut tx = 0;
        let mut ty = 0; // target
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut set_box = HashSet::<i32>::new();
        let mut set_play = HashSet::<i32>::new();
        for i in 0..m as usize {
            for j in 0..n as usize {
                if grid[i][j] == 'B' {
                    bx = i as i32;
                    by = j as i32;
                } else if grid[i][j] == 'S' {
                    px = i as i32;
                    py = j as i32;
                } else if grid[i][j] == 'T' {
                    tx = i as i32;
                    ty = j as i32;
                }
            }
        }

        // state = (x if player, y of player, x of box, y of box
        let mut queue = VecDeque::<(i32, i32, i32, i32)>::new();
        queue.push_back((px, py, bx, by));
        set_box.insert(bx * n as i32 + by);
        let mut step = 0;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let state = queue.pop_front().unwrap();
                if state.2 == tx && state.3 == ty {
                    return step as i32;
                }
                for i in 0..4 {
                    let bx = state.2 + DIRS[i];
                    let by = state.3 + DIRS[i + 1];
                    let px = state.2 - DIRS[i];
                    let py = state.3 - DIRS[i + 1];
                    if bx < 0 || bx > m as i32 - 1 || by < 0 || by > n as i32 - 1 {
                        continue;
                    }
                    if px < 0 || px > m as i32 - 1 || py < 0 || py > n as i32 - 1 {
                        continue;
                    }
                    if grid[px as usize][py as usize] != '#'
                        && grid[bx as usize][by as usize] != '#'
                        && (set_box.insert(bx * n + by) || set_play.insert(px * n + py))
                        && can_reach_pos(&grid, state.0, state.1, px, py, state.2, state.3, m, n)
                    {
                        queue.push_back((state.2, state.3, bx, by));
                    }
                }
            }
            step += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_push_box() {
        assert_eq!(
            Solution::min_push_box(vec![
                vec!['#', '#', '#', '#', '#', '#'],
                vec!['#', 'T', '#', '#', '#', '#'],
                vec!['#', '.', '.', 'B', '.', '#'],
                vec!['#', '.', '#', '#', '.', '#'],
                vec!['#', '.', '.', '.', 'S', '#'],
                vec!['#', '#', '#', '#', '#', '#']
            ]),
            3
        );
    }

    #[test]
    fn test_min_push_box_02() {
        assert_eq!(
            Solution::min_push_box(vec![
                vec!['#', '#', '#', '#', '#', '#'],
                vec!['#', 'T', '#', '#', '#', '#'],
                vec!['#', '.', '.', 'B', '.', '#'],
                vec!['#', '#', '#', '#', '.', '#'],
                vec!['#', '.', '.', '.', 'S', '#'],
                vec!['#', '#', '#', '#', '#', '#']
            ]),
            -1
        );
    }

    #[test]
    fn test_min_push_box_03() {
        assert_eq!(
            Solution::min_push_box(vec![
                vec!['#', '#', '#', '#', '#', '#'],
                vec!['#', 'T', '.', '.', '#', '#'],
                vec!['#', '.', '#', 'B', '.', '#'],
                vec!['#', '.', '.', '.', '.', '#'],
                vec!['#', '.', '.', '.', 'S', '#'],
                vec!['#', '#', '#', '#', '#', '#']
            ]),
            5
        );
    }

    #[test]
    fn test_min_push_box_04() {
        assert_eq!(
            Solution::min_push_box(vec![
                vec!['#', '#', '#', '#', '#', '#', '#'],
                vec!['#', 'S', '#', '.', 'B', 'T', '#'],
                vec!['#', '#', '#', '#', '#', '#', '#']
            ]),
            -1
        );
    }
}