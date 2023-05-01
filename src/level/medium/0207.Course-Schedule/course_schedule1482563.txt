// https://leetcode.com/problems/course-schedule/solutions/1482563/rust-dfs-memoization/
use std::collections::HashSet;

fn _is_in_cycle(node: usize, nodes: &[Vec<usize>], visited: &mut HashSet<usize>, acyclic: &mut [bool]) -> bool {
    if acyclic[node] {
        false
    } else if visited.contains(&node) {
        true
    } else {
        visited.insert(node);
        for child in &nodes[node] {
            if _is_in_cycle(*child, nodes, visited, acyclic) {
                return true;
            }
        }
        visited.remove(&node);
        acyclic[node] = true;
        false
    }
}

fn is_in_cycle(start: usize, nodes: &[Vec<usize>], acyclic: &mut [bool]) -> bool {
    let mut visited: HashSet<usize> = HashSet::new();
    _is_in_cycle(start, nodes, &mut visited, acyclic) 
}

impl Solution {
    pub fn can_finish(num_courses: i32, mut prerequisites: Vec<Vec<i32>>) -> bool {
        let mut nodes: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
        let mut i = 0;
        for edge in prerequisites {
            nodes[edge[0] as usize].push(edge[1] as usize);
            i += 1;
        }
        
        let mut acyclic: Vec<bool> = vec![false; nodes.len()];
        for i in 0..nodes.len() {
            if !acyclic[i] && is_in_cycle(i, &nodes, &mut acyclic) {
                return false;
            }
        }
        
        true
    }
}