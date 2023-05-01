// https://leetcode.com/problems/interleaving-string/solutions/3190857/rust-top-down-dp/
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut dp:Vec<Vec<Option<bool>>> = vec![vec![None;s2.len()];s1.len()];
        Solution::helper(&s1[..],&s2[..],&s3[..], &mut dp)
    }

    pub fn helper(s1:&str, s2:&str, s3:&str, dp:&mut Vec<Vec<Option<bool>>>) -> bool {
        match (s1, s2, s3) {
            ("","","") => true,
            (a, "", b) => a == b,
            ("", a, b) => a == b,
            (a,b,c) => {
                if let Some(res) = dp[s1.len()-1][s2.len()-1] {
                    return res;
                }
                let x = &a[0..1];
                let y = &b[0..1];
                let z = &c[0..1];
                let mut result = false;
                if x == z {
                    result = result || Solution::helper(&s1[1..],&s2, &s3[1..], dp);
                }
                if y == z {
                    result = result || Solution::helper(&s1, &s2[1..], &s3[1..], dp);
                }
                dp[s1.len()-1][s2.len()-1] = Some(result);
                result
            }
        }
    }
}