// https://leetcode.com/problems/jump-game-iv/solutions/3257921/rust-bfs/
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &n) in arr.iter().enumerate() {
            map.entry(n).and_modify(|v| v.push(i)).or_insert(vec![i]);
        }

        let mut q = VecDeque::new();
        q.push_back((0, 0i32));

        let mut unseen = vec![true; arr.len()];
        unseen[0] = false;

        while let Some((i, mut step)) = q.pop_front() {
            if i == arr.len() - 1 {
                return step;
            }

            step += 1;
            
            // You will get `TLE` if you keep the elements
            if let Some(v) = map.remove(&arr[i]) {
                for j in v {
                    if unseen[j] {
                        unseen[j] = false;
                        q.push_back((j, step));
                    }
                }
            }

            if i + 1 < arr.len() && unseen[i + 1] {
                unseen[i + 1] = false;
                q.push_back((i + 1, step));
            }
            if i >= 1 && unseen[i - 1] {
                unseen[i - 1] = false;
                q.push_back((i - 1, step));
            }
        }
        
        -1
    }
}