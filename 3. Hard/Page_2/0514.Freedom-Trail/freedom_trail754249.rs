// https://leetcode.com/problems/freedom-trail/solutions/754249/4ms-rust-dp/
use std::collections::HashMap;
impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut f: Vec<Vec<i64>> = vec![vec![std::i32::MAX as i64; ring.len()]; key.len()];
        let mut map = HashMap::new();
        for (i, c) in ring.char_indices() {
            map.entry(c).or_insert(Vec::new()).push(i);
        }
        let key: Vec<char> = key.chars().collect();
        for &i in map[&key[0]].iter() {
            f[0][i] = (ring.len() as i64 - i as i64).abs().min(i as i64);
        }

        for k in 1..key.len() {
            for &to in map[&key[k]].iter() {
                for &from in map[&key[k - 1]].iter() {
                    let diff = (from as i64 - to as i64).abs();
                    let diff = diff.min((ring.len() as i64 - diff).abs());
                    f[k][to] = f[k][to].min(f[k - 1][from] + diff);
                }
            }
        }
        map[&key[key.len() - 1]]
            .iter()
            .fold(std::i64::MAX, |ret, idx| ret.min(f[key.len() - 1][*idx])) as i32
            + key.len() as i32
    }
}