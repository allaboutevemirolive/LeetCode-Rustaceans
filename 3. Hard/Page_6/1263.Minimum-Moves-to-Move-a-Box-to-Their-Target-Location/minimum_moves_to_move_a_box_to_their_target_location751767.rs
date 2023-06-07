// https://leetcode.com/problems/minimum-moves-to-move-a-box-to-their-target-location/solutions/751767/idiomatic-rust-using-bfs/
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Position {
    i: usize,
    j: usize,
}

impl Position {
    pub fn new(i: usize, j: usize) -> Position {
        Position { i, j }
    }

    pub fn pos_in_dir(&self, d: Direction, grid_sizes: Position) -> Option<Position> {
        let adj_pos = match d {
            Direction::Right => Position::new(self.i, self.j + 1),
            Direction::Left => Position::new(self.i, self.j.checked_sub(1)?),
            Direction::Down => Position::new(self.i + 1, self.j),
            Direction::Up => Position::new(self.i.checked_sub(1)?, self.j),
        };

        if adj_pos.i < grid_sizes.i && adj_pos.j < grid_sizes.j {
            Some(adj_pos)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct State {
    box_pos: Position,
    player_pos: Position,
}

impl State {
    pub fn new(box_pos: Position, player_pos: Position) -> State {
        State {
            box_pos,
            player_pos,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
        }
    }
}

lazy_static! {
    static ref DIRECTIONS: Vec<Direction> = vec![
        Direction::Right,
        Direction::Left,
        Direction::Down,
        Direction::Up,
    ];
}

/// Returns a list of tuples of `(expected_player_position, moved_box_position)`.
/// If the player were to reach `expected_player_position`, the box could be moved to `moved_box_position`.
fn push_positions(grid: &Vec<Vec<Tile>>, box_pos: Position) -> Vec<(Position, Position)> {
    let mut res = vec![];

    let grid_sizes = Position::new(grid.len(), grid[0].len());
    for direction in &*DIRECTIONS {
        let new_player_pos = box_pos.pos_in_dir(direction.opposite(), grid_sizes);
        let new_box_pos = box_pos.pos_in_dir(*direction, grid_sizes);
        if let (Some(new_player_pos), Some(new_box_pos)) = (new_player_pos, new_box_pos) {
            if grid[new_player_pos.i][new_player_pos.j] == Tile::Empty
                && grid[new_box_pos.i][new_box_pos.j] == Tile::Empty
            {
                res.push((new_player_pos, new_box_pos))
            }
        }
    }

    res
}

/// Returns whether the player can move from `state.player_pos` to `new_player_pos`.
/// Player cannot move through `Tile::Wall` or `state.box_pos`.
fn can_move_player(grid: &Vec<Vec<Tile>>, state: State, new_player_pos: Position) -> bool {
    let mut worklist = vec![state.player_pos];

    let grid_sizes = Position::new(grid.len(), grid[0].len());
    let mut visited = HashSet::new();
    visited.insert(state.player_pos);
    while let Some(player_pos) = worklist.pop() {
        if player_pos == new_player_pos {
            return true;
        }

        for direction in &*DIRECTIONS {
            if let Some(adj_pos) = player_pos.pos_in_dir(*direction, grid_sizes) {
                if grid[adj_pos.i][adj_pos.j] == Tile::Empty
                    && adj_pos != state.box_pos
                    && visited.insert(adj_pos)
                {
                    worklist.push(adj_pos);
                }
            }
        }
    }

    false
}

/// Returns the minimum number of pushes to get the box from `start` to `end` given
/// the player position `player`.
fn num_pushes(
    grid: &Vec<Vec<Tile>>,
    box_pos: Position,
    end: Position,
    player_pos: Position,
) -> Option<usize> {
    let mut q = VecDeque::new();
    let init_state = State::new(box_pos, player_pos);
    q.push_back((0, init_state));

    let mut visited = HashSet::new();
    visited.insert(init_state);
    while let Some((cur_dist, state)) = q.pop_front() {
        if state.box_pos == end {
            return Some(cur_dist);
        }

        // let mut s = vec!["".to_string(); grid.len()];
        // for (i, row) in grid.iter().enumerate() {
        //     for (j, tile) in row.iter().enumerate() {
        //         if *tile == Tile::Empty {
        //             s[i].push('.');
        //         } else {
        //             s[i].push('#');
        //         }

        //         if Position::new(i, j) == state.box_pos {
        //             s[i].pop();
        //             s[i].push('B');
        //         } else if Position::new(i, j) == state.player_pos {
        //             s[i].pop();
        //             s[i].push('P');
        //         }
        //     }
        //     println!("{:?}", s[i]);
        // }
        // println!("------");

        // For each position the box can be pushed into, check if the player can move behind the box.
        for (expected_player_pos, new_box_pos) in push_positions(grid, state.box_pos) {
            let new_state = State::new(new_box_pos, expected_player_pos);
            // Order is important here! If we insert before moving, we can discount some possibilities early.
            if can_move_player(grid, state, expected_player_pos) && visited.insert(new_state) {
                q.push_back((cur_dist + 1, new_state));
            }
        }
    }

    None
}

pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
    let mut box_pos = None;
    let mut end = None;
    let mut player_pos = None;
    let mut grid_tile = vec![vec![]; grid.len()];
    for (i, row) in grid.into_iter().enumerate() {
        for (j, ch) in row.into_iter().enumerate() {
            match &ch {
                '#' => grid_tile[i].push(Tile::Wall),
                _ => grid_tile[i].push(Tile::Empty),
            }

            match ch {
                'B' => box_pos = Some(Position::new(i, j)),
                'T' => end = Some(Position::new(i, j)),
                'S' => player_pos = Some(Position::new(i, j)),
                _ => {}
            }
        }
    }

    match num_pushes(
        &grid_tile,
        box_pos.unwrap(),
        end.unwrap(),
        player_pos.unwrap(),
    ) {
        Some(x) => x as i32,
        None => -1,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_box_pushes() {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '#', '#', '#', '#'],
            vec!['#', '.', '.', 'B', '.', '#'],
            vec!['#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
        ];

        assert_eq!(min_push_box(grid), 3);
    }

    #[test]
    fn test_num_box_pushes_five_pushes() {
        let grid = vec![
            vec!['#', '#', '#', '#', '#', '#'],
            vec!['#', 'T', '.', '.', '#', '#'],
            vec!['#', '.', '#', 'B', '.', '#'],
            vec!['#', '.', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', 'S', '#'],
            vec!['#', '#', '#', '#', '#', '#'],
        ];

        assert_eq!(min_push_box(grid), 5);
    }

    #[test]
    fn test_num_box_pushes_difficult() {
        let grid = vec![
            vec!['#', '.', '.', '#', 'T', '#', '#', '#', '#'],
            vec!['#', '.', '.', '#', '.', '#', '.', '.', '#'],
            vec!['#', '.', '.', '#', '.', '#', 'B', '.', '#'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['#', '.', '.', '.', '.', '#', '.', 'S', '#'],
            vec!['#', '.', '.', '#', '.', '#', '#', '#', '#'],
        ];

        assert_eq!(min_push_box(grid), 8);
    }
}
