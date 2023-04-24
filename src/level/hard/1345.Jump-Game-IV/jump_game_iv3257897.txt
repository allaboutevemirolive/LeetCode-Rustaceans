// https://leetcode.com/problems/jump-game-iv/solutions/3257897/my-take-rust/
use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut ans = vec![n as i32; n];
        let mut visited = vec![false; n];

        visited[0] = true;
        ans[0] = 0;

        let mut mp = HashMap::new();

        for i in 1..n {
            mp.entry(arr[i]).or_insert_with(Vec::new).push(i);
        }

        let mut deque = VecDeque::with_capacity(n);
        deque.push_back(0);

        let mut ops = 1;

        while !deque.is_empty() {
            let size = deque.len();
            for _ in 0..size {
                let pos = deque.pop_front().unwrap();
                if let Some(ref v) = mp.get(&arr[pos]) {
                    for &p in v.iter() {
                        if !visited[p] {
                            ans[p] = ops;
                            visited[p] = true;
                            deque.push_back(p);
                        }
                    }

                    mp.remove(&arr[pos]);
                }

                if pos > 0 && !visited[pos - 1] {
                    ans[pos - 1] = ops;
                    visited[pos - 1] = true;
                    deque.push_back(pos - 1);
                }

                if pos + 1 < n && !visited[pos + 1] {
                    ans[pos + 1] = ops;
                    visited[pos + 1] = true;
                    deque.push_back(pos + 1);
                }
            }

            ops += 1;
        }

        ans[n - 1]
    }
}