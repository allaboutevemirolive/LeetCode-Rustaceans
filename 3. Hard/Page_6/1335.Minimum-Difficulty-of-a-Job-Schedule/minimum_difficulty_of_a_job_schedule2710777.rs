// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/solutions/2710777/rust-dp-with-hashmap/
impl Solution {
    pub fn min_difficulty(jd: Vec<i32>, d: i32) -> i32 {
        use std::{collections::HashMap, cmp::{max, min}};
        let mut cache = HashMap::new();
        
        fn dp(m: i32, i: usize, d: i32, cache: &mut HashMap<(i32, usize, i32), i32> , jd: &Vec<i32>) -> i32 {
            if d < 0 || (i == jd.len() && d != 0) {
                100_000_000
            } else if i == jd.len() {
                m
            } else {
                min(
                    match cache.get(&(max(m, jd[i]), i+1, d)) {
                        Some(&r1) => r1,
                        None => {
                            let r1 = dp(max(m, jd[i]), i+1, d, cache, jd);
                            cache.insert((max(m, jd[i]), i+1, d), r1);
                            r1
                        } 
                    },
                    m + match cache.get(&(jd[i], i+1, d-1)) { 
                        Some(&r2) => r2,
                        None => {
                            let r2 = dp(jd[i], i+1, d-1, cache, jd);
                            cache.insert((jd[i], i+1, d-1), r2);
                            r2
                        }
                    },
                )
            }
        }
        
        if jd.len() < d as usize {
            -1
        } else {
            dp(jd[0], 1, d-1, &mut cache, &jd)
        }
    }
}