// https://leetcode.com/problems/minimum-genetic-mutation/solutions/2768818/rust-bfs-and-hashset/
use std::collections::HashSet;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut hash: HashSet<Vec<u8>> = bank.into_iter()
            .map(|s| s.as_bytes().to_owned())
            .collect();
        if !hash.contains(end.as_bytes()) {
            return -1;
        }
        let types = [b'A', b'C', b'G', b'T'];
        let mut curr = vec![start.as_bytes().to_owned()];
        let mut next = Vec::new();
        let mut step = 0;
        while !curr.is_empty() {
            for mut v in curr.drain(..) {
                if v == end.as_bytes() {
                    return step;
                }
                for i in 0..v.len() {
                    let b = v[i];
                    for &t in types.iter() {
                        if t != b {
                            v[i] = t;
                            if let Some(v2) = hash.take(&v) {
                                next.push(v2);
                            }
                        }
                    }
                    v[i] = b;
                }
            }
            std::mem::swap(&mut curr, &mut next);
            step += 1;
        }
        -1
    }
}