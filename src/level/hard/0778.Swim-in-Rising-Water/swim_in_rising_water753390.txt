// https://leetcode.com/problems/swim-in-rising-water/solutions/753390/idiomatic-rust/
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};

// GRID HELPERS START

#[derive(PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    i: usize,
    j: usize,
}

impl Position {
    pub fn new(i: usize, j: usize) -> Position {
        Position { i, j }
    }

    pub fn in_direction(&self, d: &Direction, limit: &Position) -> Option<Position> {
        let adj_pos = match d {
            Direction::Left => Position::new(self.i, self.j.checked_sub(1)?),
            Direction::Right => Position::new(self.i, self.j + 1),
            Direction::Up => Position::new(self.i.checked_sub(1)?, self.j),
            Direction::Down => Position::new(self.i + 1, self.j),
        };

        if adj_pos.i < limit.i && adj_pos.j < limit.j {
            Some(adj_pos)
        } else {
            None
        }
    }

    pub fn add_i32s(&self, i: i32, j: i32) -> Option<Position> {
        Some(Position {
            i: if i < 0 {
                self.i.checked_sub(-i as usize)?
            } else {
                self.i + i as usize
            },
            j: if j < 0 {
                self.j.checked_sub(-j as usize)?
            } else {
                self.j + j as usize
            },
        })
    }

    pub fn elemwise_lt(&self, other: &Position) -> bool {
        self.i < other.i && self.j < other.j
    }
}

#[derive(PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

lazy_static! {
    pub static ref DIRECTIONS: Vec<Direction> = vec![
        Direction::Left,
        Direction::Right,
        Direction::Down,
        Direction::Up,
    ];
}

impl Direction {
    pub fn as_i32(&self) -> (i32, i32) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

// GRID HELPERS END

fn _swim_in_water(grid: Vec<Vec<i32>>, start: Position, end: Position) -> usize {
    let grid_sizes = Position::new(grid.len(), grid[0].len());

    let mut visited = HashSet::new();

    let mut q = BinaryHeap::new();
    q.push((Reverse(grid[start.i][start.j] as usize), start));
    while let Some((Reverse(cur_time), cur_pos)) = q.pop() {
        // Skip `cur_pos` if it has been visited.
        if !visited.insert(cur_pos) {
            continue;
        }

        // `cur_time` is guaranteed to be the smallest time to visit `cur_pos`.
        // So, if `cur_pos` == `end`, return early with `cur_time`.
        if cur_pos == end {
            return cur_time;
        }

        // For all neighbors, find the time to wait (may be 0), and add the (new time, new position) pair to the heap.
        for direction in &*DIRECTIONS {
            if let Some(adj_pos) = cur_pos.in_direction(direction, &grid_sizes) {
                let time_to_wait =
                    std::cmp::max(cur_time, grid[adj_pos.i][adj_pos.j] as usize) - cur_time;
                q.push((Reverse(cur_time + time_to_wait), adj_pos));
            }
        }
    }

    0
}

pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
    let start = Position::new(0, 0);
    let end = Position::new(grid.len() - 1, grid[0].len() - 1);
    _swim_in_water(grid, start, end) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swim_in_water() {
        assert_eq!(swim_in_water(vec![vec![0, 2], vec![1, 3]]), 3);
    }

    #[test]
    fn test_swim_in_water_fix_initial() {
        assert_eq!(swim_in_water(vec![vec![3, 2], vec![0, 1]]), 3);
    }

    #[test]
    fn test_swim_in_water_complex() {
        assert_eq!(
            swim_in_water(vec![
                vec![0, 1, 2, 3, 4],
                vec![24, 23, 22, 21, 5],
                vec![12, 13, 14, 15, 16],
                vec![11, 17, 18, 19, 20],
                vec![10, 9, 8, 7, 6]
            ]),
            16
        );
    }
}


impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        swim_in_water(grid)
    }
}