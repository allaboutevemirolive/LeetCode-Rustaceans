// https://leetcode.com/problems/minimum-genetic-mutation/solutions/2769743/rust-100-runtime-23-lines-easy/
use std::collections::HashSet;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut vis: HashSet<&String> = HashSet::new();

        let mut cur: HashSet<&String> = [&start].into_iter().cloned().collect();
        let mut i = 0;
        fn is_mut(s: &str, s_: &str) -> bool {
            s.chars().zip(s_.chars()).filter(|(c, c_)| c != c_).count() == 1
        }
        while !cur.is_empty() {
            if cur.contains(&end) {
                return i;
            }
            i += 1;
            vis.extend(cur.iter());
            cur = cur
                .into_iter()
                .flat_map(|s| {
                    bank.iter()
                        .filter(|s_| !vis.contains(s_))
                        .filter(move |s_| is_mut(s, s_))
                })
                .collect();
        }
        -1
    }
}