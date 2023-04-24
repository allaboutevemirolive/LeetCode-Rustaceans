// https://leetcode.com/problems/distinct-echo-substrings/solutions/561453/faster-than-100-rust-solutions-116ms-o-n-2-hashing-solution/
use std::collections::HashSet;

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        const P1 : i32 = 127;
        const M1 : i32 = 75798632;
        const P2 : i32 = 1007;
        const M2 : i32 = 665729789;
        
        let data = text.as_bytes();
        let N = data.len();
        let mut hash = vec![vec![(0, 0); N]; N];
        
        for i in 0..N {
            hash[i][i] = (data[i] as i32, data[i] as i32);
            for j in (i+1)..N {
                hash[i][j] = (
                    (hash[i][j-1].0 * P1 + data[j] as i32) % M1,
                    (hash[i][j-1].0 * P2 + data[j] as i32) % M2
                );
            }
        }
        
        let mut ans = 0;
        
        for len in 1..=(N/2) {
            let mut exists = HashSet::new();
            for i in 0..=(N-len*2) {
                if hash[i][i+len-1] == hash[i+len][i+len*2-1] {
                    exists.insert(hash[i][i+len-1]);
                }
            }
            ans += exists.len() as i32;
        }
        
        return ans;
    }
}