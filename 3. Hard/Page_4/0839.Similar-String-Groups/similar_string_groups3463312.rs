// https://leetcode.com/problems/similar-string-groups/solutions/3463312/rust-union-find-solution/
use std::collections::HashSet;
use std::iter::FromIterator;

/// Standard way to do union find
trait UnionFind {
    fn union(parents: &mut Vec<usize>, i: usize, j: usize);
    fn find(parents: &mut Vec<usize>, i: usize) -> usize;
}

impl UnionFind for Solution {
    fn find(parents: &mut Vec<usize>, i: usize) -> usize {
        if parents[i] != i {
            parents[i] = Solution::find(parents, parents[i]);
        }
        parents[i]
    }
    fn union(parents: &mut Vec<usize>, i: usize, j: usize) {
        let pi = Solution::find(parents, i);
        let pj = Solution::find(parents, j);
        parents[pi] = pj;
    }
}

impl Solution {
	/// Defination of similar string.
    pub fn is_similar_string(first: &String, second: &String) -> bool {
        match first
            .chars()
            .zip(second.chars())
            .filter(|(c1, c2)| c1 != c2)
            .collect::<Vec<_>>()
            .len()
        {
            0 | 2 => true,
            _ => false,
        }
    }
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut parents = (0..strs.len()).collect::<Vec<_>>();
        let n = strs.len();

        for i in 0..n {
            for j in i + 1..n {
                if Solution::is_similar_string(&strs[i], &strs[j]) {
                    Solution::union(&mut parents, i, j);
                }
            }
        }
        for i in 0..n {
            Solution::find(&mut parents, i);
        }
        HashSet::<usize>::from_iter(parents).len() as i32
    }
}