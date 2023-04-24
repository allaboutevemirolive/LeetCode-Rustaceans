// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/solutions/3415652/rust-bfs/
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        fn is_valid(x: i32, y: i32, grid: &Vec<Vec<i32>>) -> bool {
            x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32
        }
        use std::collections::VecDeque;
        const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut visited = vec![vec![vec![false; k as usize + 1]; grid[0].len()]; grid.len()];
        let mut queue = VecDeque::new();
        queue.push_back((0, 0, k, 0)); // x, y, remain, steps
        visited[0][0][k as usize] = true;
        while let Some((x, y, remain, steps)) = queue.pop_front() {
            if x == grid.len() as i32 - 1 && y == grid[0].len() as i32 - 1 {
                return steps;
            }
            for (nx, ny) in DIR {
                let (ux, uy) = (x + nx, y + ny);
                if is_valid(ux, uy, &grid) {
                    if grid[ux as usize][uy as usize] == 0
                        && !visited[ux as usize][uy as usize][remain as usize]
                    {
                        visited[ux as usize][uy as usize][remain as usize] = true;
                        queue.push_back((ux, uy, remain, steps + 1));
                    } else if remain > 0 && !visited[ux as usize][uy as usize][remain as usize - 1] {
                        visited[ux as usize][uy as usize][remain as usize - 1] = true;
                        queue.push_back((ux, uy, remain - 1, steps + 1));
                    }
                }
            }
        }
        -1        
    }
}