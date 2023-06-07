// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/solutions/2757941/rust-a/
use std::collections::{BinaryHeap,HashSet};
use std::cmp::Reverse;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let end = ((grid.len() - 1) as i32, (grid[0].len() -1) as i32);
        let mut heap = BinaryHeap::new();
        let mut seen:HashSet<(i32,i32,i32)> = HashSet::new();
        seen.insert((0,0,0));
        heap.push(Reverse((end.0+end.1,0,0,0,0)));
        while !heap.is_empty() {
            if let Some(Reverse((guess,dist,broken,x,y))) = heap.pop() {
                if broken <= k {
                    if (x,y) == end {
                        return dist
                    }
                    for (a,b) in [(x-1,y),(x+1,y),(x,y+1),(x,y-1)] {
                        if 0<=a && a <= end.0 && 0<=b && b <= end.1 && !seen.contains(&(broken,a,b)) {
                            seen.insert((broken,a,b));
                            heap.push(Reverse((end.0 - a + end.1 - b + dist, dist + 1, broken + grid[a as usize][b as usize], a, b)));
                        }
                    }
                }
            } 
        }
        -1
    }
}