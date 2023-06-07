// https://leetcode.com/problems/minimum-moves-to-reach-target-with-rotations/solutions/865399/rust/
use std::collections::*;

/// Indicates whether or not a spot in the grid is a wall.
type IsWall = bool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    i: usize,
    j: usize,
}

impl Pos {
    pub fn new(i: usize, j: usize) -> Pos {
        Pos { i, j }
    }

    fn in_dir(&self, dir: HeadDir) -> Pos {
        match dir {
            HeadDir::Down => Pos::new(self.i + 1, self.j),
            HeadDir::Right => Pos::new(self.i, self.j + 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HeadDir {
    Down,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Snake {
    /// The position of the snake's tail.
    tail: Pos,
    /// Where the head is pointing, relative to the tail.
    /// E.g. if tail is at `Pos {i:0, j:0}` and head_dir is `Right`, then the head is at `Pos {i: 0, j: 1}`.
    head_dir: HeadDir,
}

impl Snake {
    pub fn new(tail: Pos, head_dir: HeadDir) -> Snake {
        Snake { tail, head_dir }
    }
}

fn adj_nodes(node: Snake, grid: &Vec<Vec<IsWall>>) -> Vec<Snake> {
    /// Returns true if the given `pos` can be walked over on the given `grid.
    fn walkable(pos: Pos, grid: &Vec<Vec<IsWall>>) -> bool {
        pos.i < grid.len() && pos.j < grid[pos.i].len() && !grid[pos.i][pos.j]
    }

    let mut adjs = Vec::with_capacity(3);

    // Move right.
    adjs.push(Snake::new(node.tail.in_dir(HeadDir::Right), node.head_dir));
    // Move down.
    adjs.push(Snake::new(node.tail.in_dir(HeadDir::Down), node.head_dir));

    // Rotate clockwise / counter-clockwise.
    let rotate = match node.head_dir {
        HeadDir::Right => Snake::new(node.tail, HeadDir::Down),
        HeadDir::Down => Snake::new(node.tail, HeadDir::Right),
    };
    let corner_pos = node.tail.in_dir(HeadDir::Right).in_dir(HeadDir::Down);
    if walkable(corner_pos, grid) {
        adjs.push(rotate);
    }

    adjs.into_iter()
        .filter(|snake| {
            walkable(snake.tail, grid) && walkable(snake.tail.in_dir(snake.head_dir), grid)
        })
        .collect()
}

fn _minimum_moves(start: Snake, end: Snake, grid: Vec<Vec<IsWall>>) -> Option<usize> {
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    let mut in_q = HashSet::new();
    in_q.insert(start);

    while let Some((node, dist)) = q.pop_front() {
        // println!("{:?} {:?} {:?}", node, dist, q.len());
        if node == end {
            return Some(dist);
        }

        for adj_node in adj_nodes(node, &grid) {
            if in_q.insert(adj_node) {
                q.push_back((adj_node, dist + 1));
            }
        }
    }

    None
}

fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let grid: Vec<Vec<IsWall>> = grid
        .into_iter()
        .map(|row| row.into_iter().map(|val| val == 1).collect())
        .collect();
    let start = Snake::new(Pos::new(0, 0), HeadDir::Right);
    let end = Snake::new(Pos::new(grid.len() - 1, grid.len() - 2), HeadDir::Right);
    _minimum_moves(start, end, grid)
        .map(|x| x as i32)
        .unwrap_or(-1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snake_in_a_grid_1() {
        let grid = vec![
            vec![0, 0, 0, 0, 0, 1],
            vec![1, 1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 1, 0, 1, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0],
        ];
        assert_eq!(minimum_moves(grid), 11);
    }

    #[test]
    fn test_snake_in_a_grid_2() {
        let grid = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(minimum_moves(grid), -1);
    }
}


impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        minimum_moves(grid)
    }
}