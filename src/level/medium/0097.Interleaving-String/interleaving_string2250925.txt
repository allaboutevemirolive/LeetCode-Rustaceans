// https://leetcode.com/problems/interleaving-string/solutions/2250925/rust-non-recursive-backtracking-solution/
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() == s3.len() &&
           s1.chars().chain(s2.chars()).chain(s3.chars()).fold(0, |a, n| a ^ n as u8) == 0
        {
            let s1 = s1.as_bytes();
            let s2 = s2.as_bytes();
            let s3 = s3.as_bytes();

            let mut stack = vec![];
            let mut i  = 0;
            let mut j  = 0;
            let mut k  = 0;        
            let mut lk = 0;
            
            while k < s3.len() {
                // while s1[i] == s3[k]
                while i < s1.len() && k < s3.len() && s1[i] == s3[k] {
                    if j < s2.len() && s1[i] == s2[j] {
                        // If s1 and s2 have the same character. Push 
                        // alternative state as if s2's character had 
                        // been used (its index advanced).
                        stack.push((i, j + 1, k + 1));
                    }
                    // Advance s1's index.
                    i += 1;
                    k += 1;
                }
                // while s2[j] == s3[k]
                while j < s2.len() && k < s3.len() && s2[j] == s3[k] {
                    if i < s1.len() && s2[j] == s1[i] {
                        // If s1 and s2 have the same character. Push 
                        // alternative state as if s1's character had
                        // been used (its index advanced).
                        stack.push((i + 1, j, k + 1));
                    }
                    // Advance s2's index.
                    j += 1;
                    k += 1;
                }
                if k == lk {
                    // Last cycle was unproductive, return to the last
                    // alternative state.
                    if let Some(s) = stack.pop() {
                        i = s.0;
                        j = s.1; 
                        k = s.2;
                    } else {
					    // Fail if alternatives exhausted.
                        return false;
                    }
                }
                lk = k;
            }
            true
        } else {
            false
        }
    }
}