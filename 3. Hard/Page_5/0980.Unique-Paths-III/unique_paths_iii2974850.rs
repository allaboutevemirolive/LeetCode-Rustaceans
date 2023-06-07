// https://leetcode.com/problems/unique-paths-iii/solutions/2974850/rust-backtracking-but-no-recursion/
enum Action {
    Explore { x: usize, y: usize },
    Restore { idx: usize },
}
impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        let row_cnt = grid.len();
        let row_len = grid[0].len();
        let full = (1 << (row_cnt * row_len)) - 1;
        let (x0, y0, mut state) = grid.iter().flat_map(|row| row.iter()).enumerate().fold((0,0,0), |(x0, y0, state), (idx, el)| match *el {
            -1 => (x0, y0, state | (1 << idx)),
            1 => (idx / row_len, idx % row_len, state),
            _ => (x0, y0, state)
        });
        let mut result = 0;
        let mut stack = vec![Action::Explore{ x: x0, y: y0 }];
        while let Some(action) = stack.pop() {
            match action {
                Action::Explore { x, y } => {
                    let idx = x*row_len + y;
                    if x >= row_cnt || y >= row_len || state & (1 << idx) > 0 {
                        continue;
                    }
                    state |= 1 << idx;
                    if grid[x][y] == 2 {
                        if state==full {
                            result += 1;
                        }
                        state &= !(1 << idx);
                        continue;
                    }
                    stack.push(Action::Restore { idx });
                    if x > 0 {
                        stack.push(Action::Explore { x: x - 1, y });
                    }
                    stack.push(Action::Explore { x, y: y + 1 });
                    stack.push(Action::Explore { x: x + 1, y });
                    if y > 0 {
                        stack.push(Action::Explore { x, y: y - 1 });
                    }
                },
                Action::Restore { idx } => state &= !(1 << idx)
            }
        }

        result
    }
}
