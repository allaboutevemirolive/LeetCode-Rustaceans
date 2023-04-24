// https://leetcode.com/problems/escape-a-large-maze/solutions/874695/rust-translated-8ms-100/
impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        use std::collections::HashSet;
        fn bfs(
            visited: &mut HashSet<(i32, i32)>,
            source: (i32, i32),
            target: (i32, i32),
            blocks: usize,
        ) -> bool {
            const DIRS: [i32; 5] = [0, 1, 0, -1, 0];
            let mut q: Vec<(i32, i32)> = vec![source];
            while !q.is_empty() && q.len() <= blocks {
                let mut q1 = Vec::<(i32, i32)>::new();
                for &(x0, y0) in &q {
                    for d in 0..4 {
                        let x = x0 + DIRS[d];
                        let y = y0 + DIRS[d + 1];
                        if !(x >= 0 && x < 1_000_000 && y >= 0 && y < 1_000_000) {
                            continue;
                        }
                        if x == target.0 && y == target.1 {
                            return true;
                        }
                        if visited.insert((x, y)) {
                            q1.push((x, y));
                        }
                    }
                }
                std::mem::swap(&mut q, &mut q1);
            }
            !q.is_empty()
        }
        let mut visited_source = blocked
            .iter()
            .filter(|&v| (v[0] - source[0]).abs() + (v[1] - source[1]).abs() < 400)
            .map(|v| (v[0], v[1]))
            .collect::<HashSet<(i32, i32)>>();
        let mut visited_target = blocked
            .iter()
            .filter(|&v| (v[0] - target[0]).abs() + (v[1] - target[1]).abs() < 400)
            .map(|v| (v[0], v[1]))
            .collect::<HashSet<(i32, i32)>>();
        let source_blocks = visited_source.len();
        let target_blocks = visited_target.len();
        let source = (source[0], source[1]);
        let target = (target[0], target[1]);
        bfs(&mut visited_source, source, target, source_blocks)
            && bfs(&mut visited_target, target, source, target_blocks)
    }
}