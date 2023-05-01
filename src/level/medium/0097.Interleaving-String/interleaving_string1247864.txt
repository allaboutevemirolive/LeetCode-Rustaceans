// https://leetcode.com/problems/interleaving-string/solutions/1247864/rust-bfs/
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1_b = s1.as_bytes();
        let s2_b = s2.as_bytes();
        let s3_b = s3.as_bytes();
        let n = s1_b.len();
        let m = s2_b.len();
        if n + m != s3_b.len() {
            return false;
        }
        if n == 0 && m == 0 {
            return true;
        }

        let mut que = VecDeque::new();
        let mut cur_len = 0;
        let mut visited = HashSet::new();
        que.push_back((0, 0));
        while let Some((i1, i2)) = que.pop_front() {
            let i3 = i1 + i2;

            if i3 > cur_len {
                cur_len = i3;
                visited.clear();
            }
            if i1 < n && s3_b[i3] == s1_b[i1] && !visited.contains(&(i1+1, i2)) {
                if (i1 + 1) == n && i2 == m {
                    return true
                }
                visited.insert((i1 + 1, i2));
                que.push_back((i1 + 1, i2));
            }
            if i2 < m && s3_b[i3] == s2_b[i2] && 
                !visited.contains(&(i1, i2 + 1)) {
                if i1 == n && (i2 + 1) == m {
                    return true
                }
                visited.insert((i1, i2 + 1));
                que.push_back((i1, i2 + 1));
            }
        }
        return false;
        
        
    }
}