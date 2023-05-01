// https://leetcode.com/problems/interleaving-string/solutions/921711/dp-solution-in-rust/
impl Solution {
        pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (l1, l2, l3) = (s1.len(), s2.len(), s3.len()); 
        if l1 + l2 != l3 {
            return false;
        }

        let (s1, s2, s3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp: Vec<Vec<bool>> = vec![vec![false; l2+1]; l1+1];
        dp[0][0] = true;

        for i in 1..=l1 {
            if s1[i-1] == s3[i-1] {
                dp[i][0] = dp[i-1][0];
            }
        }

        for j in 1..=l2 {
            if s2[j-1] == s3[j-1] {
                dp[0][j] = dp[0][j-1];
            }
        }

        for i in 1..=l1 {
            for j in 1..=l2 {
                dp[i][j] = (dp[i-1][j] && s3[i+j-1] == s1[i-1]) ||
                    (dp[i][j-1] && s3[i+j-1] == s2[j-1]);
            }
        }
        dp[l1][l2]
    }
}