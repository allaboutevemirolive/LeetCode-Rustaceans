// https://leetcode.com/problems/interleaving-string/solutions/2251377/rust-dp-seeking-entry-level-employment/
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        // memo[s1][s2]
        let mut memo = vec![vec![None; s2.len() + 1]; s1.len() + 1];
        Solution::my_interleave(&s1, &s2, &s3, &mut memo)
    }
    
    pub fn my_interleave(s1: &str, s2: &str, s3: &str, 
        memo: &mut Vec<Vec<Option<bool>>>) -> bool {
        if s1.is_empty() && s2.is_empty() && s3.is_empty() {
            return true
        } else if s1.is_empty() && s2.is_empty() && !s3.is_empty() {
            return false
        } else if (!s1.is_empty() || !s2.is_empty()) && s3.is_empty() {
            return false
        }
        if let Some(b) = memo[s1.len()][s2.len()] {
            return b
        }
        
        let mut is_good = false; 
        if !s1.is_empty() && !s3.is_empty() 
            && s1.chars().next().unwrap() == s3.chars().next().unwrap() {
                if Solution::my_interleave(&s1[1..], s2, &s3[1..], memo) {
                    is_good = true; 
                }
        }
        if !s2.is_empty() && !s3.is_empty() 
            && s2.chars().next().unwrap() == s3.chars().next().unwrap() {
                if Solution::my_interleave(s1, &s2[1..], &s3[1..], memo) {
                    is_good = true; 
                }
        }
        memo[s1.len()][s2.len()] = Some(is_good);

        is_good
    }
}