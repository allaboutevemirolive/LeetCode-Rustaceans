// https://leetcode.com/problems/cut-off-trees-for-golf-event/solutions/211721/sort-bfs-in-rust/
use std::collections::VecDeque;
use std::collections::HashSet;

const DIRECTIONS: [i32; 5] = [1, 0, -1, 0, 1];

impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let N = forest.len();
        let M = forest[0].len();
        let mut trees = Vec::new();
		// identify all trees
        for x in 0..N {
            for y in 0..M {
                if forest[x][y] < 2 {
                    continue
                }
                trees.push((x, y));
            }
        }
		// sort them by their height
        trees.sort_unstable_by(|x, y| forest[x.0][x.1].cmp(&forest[y.0][y.1]));
        
        let distance_between = |x: (usize, usize), y: (usize, usize)| -> Option<i32> {
            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            let mut seen: HashSet<(usize, usize)> = HashSet::new();
            queue.push_back(x.clone());
            seen.insert(x);
            
            let mut distance = 0;
            
            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let loc = queue.pop_front().unwrap();
                    if loc == y {
                        return Some(distance);
                    }
                    for dir in 0..DIRECTIONS.len()-1 {
                        let next_loc = (loc.0 as i32 + DIRECTIONS[dir], loc.1 as i32 + DIRECTIONS[dir+1]);
                        if next_loc.0 < 0 || next_loc.0 >= N as i32 {
                            continue;
                        }
                        if next_loc.1 < 0 || next_loc.1 >= M as i32 {
                            continue;
                        }
                        let next_loc = (next_loc.0 as usize, next_loc.1 as usize);
                        if forest[next_loc.0][next_loc.1] < 1 {
                            continue;
                        }
                        if seen.contains(&next_loc) {
                            continue;
                        }
                        seen.insert(next_loc.clone());
                        queue.push_back(next_loc);
                    }
                }
                distance += 1;
            }
            
            None
        };
        
        let mut steps = 0;
        let mut last = (0, 0);
		// sum up distances between each location
        for tree in &trees {
            if let Some(distance) = distance_between(last, tree.clone()) {
                steps += distance;
            } else {
                return -1;
            }
            last = tree.clone();
        }
        
        steps
    }
}