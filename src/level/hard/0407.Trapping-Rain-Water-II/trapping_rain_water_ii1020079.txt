// https://leetcode.com/problems/trapping-rain-water-ii/solutions/1020079/rust-clean-idiomatic-code/
use std::collections::{HashSet, BinaryHeap};
use std::cmp::Reverse;

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Pos {
    i: usize,
    j: usize,
}
impl Pos {
    pub fn new(i: usize, j: usize) -> Pos {
        Pos { i, j }
    }

    pub fn from_isize_limits(i: isize, j: isize, limits: Pos) -> Option<Pos> {
        if i < 0 || j < 0 || (i as usize) >= limits.i || (j as usize) >= limits.j {
            None
        } else {
            Some(Pos::new(i as usize, j as usize))
        }
    }
}

fn _trap_rain_water(heights: Vec<Vec<i32>>) -> i32 {
    if heights.is_empty() || heights[0].is_empty() {
        return 0;
    }
    
    let mut seen = HashSet::new();
    let mut q = BinaryHeap::new();
    
    let insert = |pos: Pos, seen: &mut HashSet<Pos>, q: &mut BinaryHeap<(Reverse<i32>, Pos)>| {
        if seen.insert(pos) {
            q.push((Reverse(heights[pos.i][pos.j]), pos));
        }
    };
    // Insert the top and bottom rows of the height map.
    for j in 0..heights[0].len() {
        insert(Pos::new(0, j), &mut seen, &mut q);
        insert(Pos::new(heights.len() - 1, j), &mut seen, &mut q);
    }
    // Insert the left and right edges (without duplicating the corners).
    for i in 1..(heights.len() - 1) {
        insert(Pos::new(i, 0), &mut seen, &mut q);
        insert(Pos::new(i, heights[0].len() - 1), &mut seen, &mut q);
    }
    
    let mut water = 0;
    let mut max_height = 0;
    let limits = Pos::new(heights.len(), heights[0].len());
    while let Some((Reverse(height), pos)) = q.pop() {
        max_height = std::cmp::max(max_height, height);
        water += max_height - height;
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let (i, j) = (pos.i as isize + di, pos.j as isize + dj);
            if let Some(adj_pos) = Pos::from_isize_limits(i, j, limits) {
                insert(adj_pos, &mut seen, &mut q);
            }
        }
    }
    
    water
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        _trap_rain_water(height_map)
    }
}