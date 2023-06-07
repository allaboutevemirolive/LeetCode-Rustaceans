// https://leetcode.com/problems/decode-ways-ii/solutions/807422/rust-translated-0ms-100/
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut e0 = 1i64;
        let mut e1 = 0i64;
        let mut e2 = 0i64;
        let mut f0 = e0;
        let mut f1 = e1;
        let mut f2 = e2;
        for &c in s.as_bytes() {
            if c == b'*' {
                f0 = 9 * e0 + 9 * e1 + 6 * e2;
                f1 = e0;
                f2 = e0;
            } else {
                f0 = if c > b'0' { e0 } else { 0 } + e1 + if c <= b'6' { e2 } else { 0 };
                f1 = if c == b'1' { e0 } else { 0 };
                f2 = if c == b'2' { e0 } else { 0 };
            }
            e0 = f0 % MOD;
            e1 = f1;
            e2 = f2;
        }
        e0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_decodings() {
        assert_eq!(Solution::num_decodings("*".to_string()), 9)
    }

    #[test]
    fn test_num_decodings_02() {
        assert_eq!(Solution::num_decodings("1*".to_string()), 18)
    }
}