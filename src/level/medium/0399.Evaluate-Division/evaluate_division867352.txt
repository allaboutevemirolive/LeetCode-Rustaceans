// https://leetcode.com/problems/evaluate-division/solutions/867352/rust-dfs-solution/
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut hm: HashMap<&str, HashMap<&str, f64>> = HashMap::new();
        for (i, equation) in equations.iter().enumerate() {
            hm.entry(&equation[0])
                .or_insert_with(HashMap::new)
                .insert(&equation[1], values[i]);
            hm.entry(&equation[1])
                .or_insert_with(HashMap::new)
                .insert(&equation[0], 1.0 / values[i]);
        }
        let mut answer = Vec::with_capacity(queries.len());
        for query in queries.iter() {
            let mut hs: HashSet<String> = HashSet::new();
            answer.push(
                if let Some(ret) = Solution::dfs(&hm, &query[0], &query[1], &mut hs) {
                    ret
                } else {
                    -1.0
                },
            );
        }
        answer
    }
    fn dfs(
        hm: &HashMap<&str, HashMap<&str, f64>>,
        src: &str,
        dst: &str,
        hs: &mut HashSet<String>,
    ) -> Option<f64> {
        if let Some(m) = hm.get(src) {
            for e in m.iter() {
                if e.0 == &dst {
                    return Some(*e.1);
                }
                if hs.contains(&e.0.to_string()) {
                    continue;
                }
                hs.insert(src.to_string());
                if let Some(ret) = Solution::dfs(hm, &e.0, dst, hs) {
                    return Some(*e.1 * ret);
                }
                hs.remove(src);
            }
        }
        None
    }
}