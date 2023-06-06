// https://leetcode.com/problems/distinct-subsequences/solutions/3504983/rust-very-short/
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let mut answer: Vec<i32> = vec![0; t.len() + 1];
        answer[0] = 1;
        for x in s {
            for (i, y) in t.iter().copied().enumerate().rev() {
                if (x == y) {
                    answer[i+1] += answer[i];
                }
            }
        }
        return answer[t.len()];
    }
}