// https://leetcode.com/problems/unique-paths-iii/solutions/599142/rust-0ms-dfs-backtracking/
impl Solution {
    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        // Compress a bit
        let gs: Vec<Vec<Cell>> = grid.into_iter()
            .map(|r| r.into_iter().map(|v| Cell::from_i32(v))
                .collect())
            .collect();
        let mut g = Grid::new(gs);
        solve_by_dfs(&mut g) as i32
    }
}

#[repr(i8)]
#[derive(Eq, PartialEq, Copy, Clone)]
enum Cell {
    Obstacle = -1,
    Empty,
    Begin,
    End,
    Visited,
}

use Cell::*;

impl Cell {
    fn can_walk_on(self) -> bool {
        match self {
            Empty | Begin | End => true,
            _ => false,
        }
    }

    fn from_i32(v: i32) -> Self {
        unsafe { ::std::mem::transmute(v as i8) }
    }
}

struct Grid {
    gs: Vec<Vec<Cell>>,
    w: usize,
    h: usize,
}

type Pos = (i32, i32);

// Bunch of helpers
impl Grid {
    fn new(gs: Vec<Vec<Cell>>) -> Self {
        let w = gs[0].len();
        let h = gs.len();
        Self {
            gs,
            w,
            h,
        }
    }

    fn neighbours(&self, p: Pos) -> [Pos; 4] {
        let (x, y) = p;
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
    }

    fn poses(&self) -> impl Iterator<Item=Pos> {
        let w = self.w as i32;
        let h = self.h as i32;
        (0..w).flat_map(move |x| {
            (0..h).map(move |y| {
                (x, y)
            })
        })
    }

    fn get(&self, p: Pos) -> Cell {
        let (x, y) = p;
        if x < 0 || y < 0 || x as usize >= self.w || y as usize >= self.h {
            Obstacle
        } else {
            self.gs[y as usize][x as usize]
        }
    }

    fn set(&mut self, p: Pos, v: Cell) {
        let (x, y) = p;
        self.gs[y as usize][x as usize] = v;
    }

    fn find(&self, what: Cell) -> Option<Pos> {
        self.poses().find(|p| self.get(*p) == what)
    }

    fn count_of_non_obstacle(&self) -> usize {
        return self.poses()
            .filter(|p| self.get(*p) != Obstacle)
            .count()
    }
}

fn solve_by_dfs(g: &mut Grid) -> usize {
    let non_obstacles = g.count_of_non_obstacle();
    let begin = g.find(Begin).unwrap();
    dfs(g, begin, 0, non_obstacles)
}

// DFS by returning the number of paths reachable from `to`.
fn dfs(g: &mut Grid, to: Pos, count: usize, count_goal: usize) -> usize {
    let v = g.get(to);
    let count = count + 1;
    match v {
        Empty | Begin => {
            g.set(to, Visited);

            let mut res = 0_usize;
            for n in g.neighbours(to).iter() {
                if !g.get(*n).can_walk_on() {
                    continue;
                }
                res += dfs(g, *n, count, count_goal);
            }
            // Backtrack
            g.set(to, v);
            res
        }
        End => {
            // Make sure we covered all cells.
            if count == count_goal {
                1
            } else {
                0
            }
        }
        _ => panic!("Not reachable"),
    }
}