// https://leetcode.com/problems/minimum-cost-to-make-at-least-one-valid-path-in-a-grid/solutions/709086/rust-bfs/
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i32, // col
    y: i32, // row
}

#[derive(Debug, Default, Clone)]
struct PositionCost {
    x: i32, // col
    y: i32, // row
    cost: i32,
}

pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let m = grid.len(); // col
    let n = grid[0].len(); // row
    if m <= 1 && n <= 1 {
        return 0;
    };
    let mut queue = VecDeque::new();
    queue.push_back(PositionCost::default());
    let mut visited = HashSet::<Position>::new();
    while !queue.is_empty() {
        let t = queue.pop_front().unwrap();
        visited.insert(Position { x: t.x, y: t.y });
        ans = t.cost;
        if t.x == n as i32 - 1 && t.y == m as i32 - 1 {
            return ans as i32;
        };
        let row = t.y as usize; // row
        let col = t.x as usize; // col
        for direction in 1..=4 {
            let mut x = t.x;
            let mut y = t.y;
            match direction {
                1 => x += 1,
                2 => x -= 1,
                3 => y += 1,
                4 => y -= 1,
                _ => {} // never reach here
            }
            if x < 0 || x >= n as i32 || y < 0 || y >= m as i32 {
                continue;
            };
            let pos = Position { x, y };
            if visited.contains(&pos) {
                continue;
            };
            let cost = if grid[row][col] == direction { 0 } else { 1 };
            let pc = PositionCost {
                x,
                y,
                cost: t.cost + cost,
            };
            if cost == 1 {
                queue.push_back(pc);
            } else {
                queue.push_front(pc);
            }
        }
    }
    ans as i32
}
