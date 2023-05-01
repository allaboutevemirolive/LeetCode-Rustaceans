// https://leetcode.com/problems/combination-sum/solutions/2127430/rust-recursive-dfs/
use std::cmp::Ordering;

impl Solution {
    fn dfs(candidates: &[i32], i: usize, target: i32, curr: &mut Vec<i32>, sum: i32, mut rez: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        match sum.cmp(&target) {
            Ordering::Less => {
                for (j, candidate) in candidates.iter().enumerate().skip(i) {
                    curr.push(*candidate);
                    rez = Self::dfs(candidates, j, target, curr, sum + candidate, rez);
                    curr.pop();
                }
            },
            Ordering::Equal => {
                rez.push(curr.clone());
            },
            Ordering::Greater => (),
        }
        rez
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Self::dfs(&candidates, 0, target, &mut vec![], 0, vec![])
    }
}