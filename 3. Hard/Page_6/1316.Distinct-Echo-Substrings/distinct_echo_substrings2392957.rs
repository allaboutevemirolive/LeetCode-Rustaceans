// https://leetcode.com/problems/distinct-echo-substrings/solutions/2392957/rust-solution-using-rolling-hash/
use std::collections::*;
impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        const MOD:usize = 2usize.pow(61)-1;  // big prime number

        let n = text.len();
        let s = text.chars().collect::<Vec<char>>();
        let s = s.into_iter().map(|v| (v as u8 - 'a' as u8) as usize).collect::<Vec<usize>>();

        let value_pattern = 26; // a-z
        // memo[start][end]
        let mut memo = vec![vec![0;n];n];
        for i in 0..n {
            let mut hash = 0usize;
            for j in i..n {
                let c = s[j];
                hash = (hash * (value_pattern+1) + c+1) % MOD;
                memo[i][j] = hash;
            }
        }

        let mut set = HashSet::new();
        for firat_start in 0..n {
            for first_end in firat_start..n {
                let len = first_end - firat_start + 1;
                let second_start = first_end+1;
                let second_end = first_end+len;
                if n <= second_end { break }
                let v1 = memo[firat_start][first_end];
                let v2 = memo[second_start][second_end];
                if v1 == v2 {
                    set.insert(v1);
                }
            }
        }

        set.len() as i32
    }
}