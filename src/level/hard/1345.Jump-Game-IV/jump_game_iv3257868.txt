// https://leetcode.com/problems/jump-game-iv/solutions/3257868/rust-straightforward-floodfill-tc-o-n-39-ms-sc-o-n-7-2mb/
use std::collections::HashMap;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        if n <= 1 { return 0; }
        if n == 2 || arr[0] == arr[n-1] { return 1; }
        let mut index: HashMap<i32,Vec<usize>> = HashMap::new();
        for i in 0 .. n {
            index.entry(arr[i]).or_default()
                .push(i);
        }
        let mut unseen = vec![true; n];
        let mut generations = 0;
        let mut active = vec![];
        let mut future = vec![];
        active.push(0);
        while !active.is_empty() {
            for i in active.drain(..) {
                if i == n-1 {
                    return generations;
                }
                // Possible next steps:
                // 1. Neighbor, earlier
                if i > 0 && unseen[i-1] {
                    future.push(i-1);
                    unseen[i-1] = false;
                }
                // 2. Neighbor, later
                if i+1 < n && unseen[i+1] {
                    future.push(i+1);
                    unseen[i+1] = false;
                }
                // 3. Matching numbers
                if let Some(v) = index.remove(&arr[i]) {
                    for ii in v {
                        if unseen[ii] {
                            future.push(ii);
                            unseen[ii] = false;
                        }
                    }
                }
            }
            std::mem::swap(&mut active, &mut future);
            generations += 1;
        }
        unreachable!();
    }
}