// https://leetcode.com/problems/powx-n/solutions/2213788/rust-recursive-tail-call-optimization/
impl Solution {
    fn recurse_my_pow(rez: f64, factor: f64, n: i64) -> f64 {
        if n == 0 {
            rez
        } else {
            Self::recurse_my_pow(
                if n & 1 == 1 { rez * factor } else { rez },
                factor * factor,
                n >> 1,
            )
        }
    }

    pub fn my_pow(x: f64, n: i32) -> f64 {
        Self::recurse_my_pow(
            1.0,
            if x == 0.0 { 0.0 } else if n >= 0 { x } else { 1.0 / x },
            if n >= 0 { n as i64 } else { -(n as i64) },
        )
    }
}