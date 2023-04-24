// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/solutions/2757917/rust-bfs/
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let height = grid.len() as i32;
        let width = grid[0].len() as i32;
        let mut curr: Vec<(usize, usize)> = vec![(0, 0)];
        let mut next: Vec<(usize, usize)> = Vec::new();
        let mut visited = vec![vec![k + 1; grid[0].len()]; grid.len()];
        visited[0][0] = 0;
        let dir = [0, 1, 0, -1, 0];
        for step in 0.. {
            if curr.is_empty() {
                break;
            }
            for (x, y) in curr.drain(..) {
                if x as i32 == height - 1 && y as i32 == width - 1 {
                    return step; // goal
                }
                for d in 0..4 {
                    let x2 = x as i32 + dir[d];
                    let y2 = y as i32 + dir[d + 1];
                    if x2 < 0 || x2 >= height
                    || y2 < 0 || y2 >= width {
                        continue; // out of bound
                    }
                    let x2 = x2 as usize;
                    let y2 = y2 as usize;
                    if visited[x2][y2] <= visited[x][y] {
                        continue; // visited
                    }
                    if grid[x2][y2] == 1 {
                        if visited[x][y] == k {
                            continue; // cannot break
                        }
                        visited[x2][y2] = visited[x][y] + 1;
                    }
                    else {
                        visited[x2][y2] = visited[x][y];
                    }
                    next.push((x2, y2));
                }
            }
            std::mem::swap(&mut curr, &mut next);
        }
        -1
    }
}