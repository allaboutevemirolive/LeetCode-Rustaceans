// https://leetcode.com/problems/shortest-path-to-get-all-keys/solutions/839020/rust-translated-24ms-100/
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct State {
    i: i32,
    j: i32,
    keys: i32,
}

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        const DIRS: [i32; 5] = [0, -1, 0, 1, 0];

        let m = grid.len();
        let n = grid[0].len();
        let mut i = -1i32;
        let mut j = -1i32;
        let mut max = 0;
        for x in 0..m {
            for y in 0..n {
                let c = grid[x].as_bytes()[y];
                if c == b'@' {
                    i = x as i32;
                    j = y as i32;
                } else if c >= b'a' && c <= b'f' {
                    max = std::cmp::max(max, c - b'a' + 1);
                }
            }
        }
//        println!("max = {}, i = {}, j = {}", max, i, j);
        let mut all_keys = (1 << max) - 1;
//        println!("{}", all_keys);
        let mut visited = HashSet::<State>::new();
        let mut queue = VecDeque::<State>::new();
        let start = State { i, j, keys: 0 };
        visited.insert(start.clone());
        queue.push_back(start);
        let mut step = 0;
        while !queue.is_empty() {
            let mut size = queue.len();
            while size > 0 {
                size -= 1;
                let curr = queue.pop_front().unwrap();
                if curr.keys == all_keys {
                    return step;
                }
                for k in 0..4 {
                    let x = curr.i + DIRS[k];
                    let y = curr.j + DIRS[k + 1];
                    if x < 0 || x >= m as i32 || y < 0 || y >= n as i32 {
                        continue;
                    }
                    let c = grid[x as usize].as_bytes()[y as usize];
                    if c == b'#' {
                        continue;
                    }
                    if c >= b'A' && c <= b'F' && (curr.keys & (1 << (c - b'A'))) == 0 {
                        continue;
                    }
                    let mut keys = curr.keys;
                    if c >= b'a' && c <= b'f' {
                        keys |= (1 << (c - b'a'));
                    }
                    let next = State { i: x, j: y, keys };
                    if visited.contains(&next) {
                        continue;
                    }
                    visited.insert(next.clone());
                    queue.push_back(next);
                }
            }
            step += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path_all_keys() {
        assert_eq!(
            Solution::shortest_path_all_keys(vec![
                "@.a.#".to_owned(),
                "###.#".to_owned(),
                "b.A.B".to_owned()
            ]),
            8
        );
    }

    #[test]
    fn test_shortest_path_all_keys_02() {
        assert_eq!(
            Solution::shortest_path_all_keys(vec![
                "@..aA".to_owned(),
                "..B#.".to_owned(),
                "....b".to_owned()
            ]),
            6
        );
    }
}