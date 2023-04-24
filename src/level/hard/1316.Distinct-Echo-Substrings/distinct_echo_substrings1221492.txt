// https://leetcode.com/problems/distinct-echo-substrings/solutions/1221492/rust-brute-force-o-n-3-beats-100-100-nevertheless/
use std::collections::HashSet;

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let bytes = text.as_bytes();
        let n = bytes.len();
        let mut ans = HashSet::new();
        let mut cnt = 0;
        for l in 1..=n/2 {
            for start in l..=n-l {
                let seg = &bytes[start..start+l];
                if &bytes[start-l..start] == seg {
                    ans.insert(seg.clone());
                }
            }
            cnt += ans.len();
            ans.clear();
        }
        cnt as i32
    }
}