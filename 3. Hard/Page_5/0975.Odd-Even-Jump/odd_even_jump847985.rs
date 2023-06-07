// https://leetcode.com/problems/odd-even-jump/solutions/847985/accepted-rust-solution/
    pub fn odd_even_jumps(a: Vec<i32>) -> i32 {
		use std::collections::{BTreeMap, Bound};
        fn get_lt<'a, K: Ord, V>(map: &'a BTreeMap<K, V>, key: &K) -> Option<(&'a K, &'a V)> {
            map.range((Bound::Unbounded, Bound::Included(key))).next_back()
        }

        fn get_gt<'a, K: Ord, V>(map: &'a BTreeMap<K, V>, key: &K) -> Option<(&'a K, &'a V)> {
            map.range((Bound::Included(key), Bound::Unbounded)).next()
        }

        let mut lower = vec![false; a.len()];
        let mut higher = vec![false; a.len()];
        *lower.last_mut().unwrap() = true;
        *higher.last_mut().unwrap() = true;
        let mut pos = BTreeMap::new();
        pos.insert(*a.last().unwrap(), a.len() - 1);
        let mut res = 1;
        for (i, n) in a.iter().enumerate().take(a.len() - 1).rev() {
            if let Some((_, j)) = get_lt(&pos, n) {
                lower[i] = higher[*j];
            }
            if let Some((_, j)) = get_gt(&pos, n) {
                higher[i] = lower[*j];
                if higher[i] {
                    res += 1;
                }
            }
            pos.insert(*n, i);
        }
        res
    }