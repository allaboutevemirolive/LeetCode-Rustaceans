// https://leetcode.com/problems/powx-n/solutions/2901477/rust-binary-pow-algorithm/
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn pow(x: f64, n: i32) -> f64 {
            if n == 0 {
                return 1.0;
            }
            let u = pow(x, n / 2);
            if n % 2 == 0 {
                u * u
            } else {
                x * u * u
            }
        }
        let p = pow(x, n.abs());
        if n < 0 {
            1.0 / p
        } else {
            p
        }
    }
}