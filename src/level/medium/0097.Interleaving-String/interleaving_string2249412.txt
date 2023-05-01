// https://leetcode.com/problems/interleaving-string/solutions/2249412/rust-dp-o-s2-length-space-with-iterators/
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut dp = vec![true; s2.len() + 1];
        let mut dp2 = vec![true; s2.len() + 1];
        for (j, (c2, c3)) in s2.chars().zip(s3.chars()).enumerate() {
            dp[j + 1] = dp[j] && c2 == c3;
        }
        for (i, c1) in s1.chars().enumerate() {
            let mut it3 = s3[i..=i + s2.len()].chars();
            let c3 = it3.next().unwrap(); // needs to be in separate line?
            dp2[0] = dp[0] && c1 == c3;
            for (j, (c2, c3)) in s2.chars().zip(it3).enumerate() {
                dp2[j + 1] = (dp2[j] && c2 == c3) || (dp[j + 1] && c1 == c3);
            }
            std::mem::swap(&mut dp, &mut dp2);
        }
        dp[s2.len()]
    }
}