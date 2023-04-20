// https://leetcode.com/problems/cracking-the-safe/solutions/892038/rust-translated-0ms-100/
impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        let m = k.pow((n - 1) as u32);
        let mut dp = vec![0i32; (m * k) as usize];
        for i in 0..k as usize {
            for q in 0..m as usize {
                dp[i * m as usize + q] = (q * k as usize + i) as i32;
            }
        }
        let mut ans = String::new();
        for i in 0..(m * k) as usize {
            let mut j = i;
            while dp[j] >= 0 {
                ans.push(((j as i32 / m) as u8 + b'0') as char);
                let v = dp[j];
                dp[j] = -1;
                j = v as usize;
            }
        }
        for _ in 0..n as usize - 1 {
            ans.push('0');
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crack_safe() {
        assert_eq!(Solution::crack_safe(1, 2), "01".to_string());
    }

    #[test]
    fn test_crack_safe_02() {
        assert_eq!(Solution::crack_safe(2, 2), "00110".to_string());
    }
}