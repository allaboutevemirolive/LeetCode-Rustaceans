// https://leetcode.com/problems/number-of-squareful-arrays/solutions/537277/rust-bfs-0ms-2-1m/
use std::collections::{ VecDeque, HashSet }; 

impl Solution {
    pub fn num_squareful_perms(a: Vec<i32>) -> i32 {
        let mut map = vec![vec![]; a.len()];
        let mut visited = HashSet::new();
        fn is_perfect_square(x: i32) -> bool {
            ((x as f64).sqrt() as i32).pow(2) == x
        }
        
        for i in 0..a.len() {
            for j in 0..a.len() {
                if i == j { continue; }
                
                if is_perfect_square(a[i] + a[j]) {
                    map[i].push(j);
                }
            }
        }
                
        let mut q = VecDeque::new();
        for i in 0..a.len() { q.push_back(vec![i]); }
        
        let mut res = 0;
        
        while let Some(sub) = q.pop_front() {
            if sub.len() == a.len() {
                res += 1;
                continue;
            }
            
            let options = &map[sub[sub.len()-1]];
            for v in options {
                if sub[0..].contains(&v) {
                    continue;
                }
                
                let mut next = sub[0..].to_vec();
                next.push(*v);
                
                let footprint = format!("{:?}", next.iter().map(|x| a[*x]).collect::<Vec<i32>>());
                
                if !visited.contains(&footprint) {
                    visited.insert(footprint);
                    q.push_back(next);
                }
            }
        }
        
        res
    }
}