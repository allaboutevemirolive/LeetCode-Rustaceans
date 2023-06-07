// https://leetcode.com/problems/jump-game-iv/solutions/1691641/rust-solution/
use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        let mut g: HashMap<_, Vec<_>> = HashMap::new();
        for (i, x) in arr.iter().enumerate() {
            g.entry(x).or_default().push(i);
        }

        let mut q = VecDeque::with_capacity(n);
        q.push_back((0, 0));
        while let Some((i, d)) = q.pop_front() {
            if i == n - 1 {
                return d;
            }
            let d = d + 1;
            let mut add = |x| {
                if was.insert(x) {
                    q.push_back((x, d));
                }
            };
            add(i + 1);
            if i >= 1 {
                add(i - 1);
            }
            for x in g.remove(&arr[i]).into_iter().flatten() {
                add(x);
            }
        }
        unreachable!();
    }
}