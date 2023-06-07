// https://leetcode.com/problems/equal-rational-numbers/solutions/214660/rust-cheating-solution/
fn to_float(s: &str) -> f64 {
    match s.find("(") {
        None => s.parse().unwrap(),
        Some(p) => 
            format!("{}{}", &s[..p], 
                std::iter::repeat(&s[p+1..s.len()-1])
                .take(10).collect::<String>())
            .parse().unwrap()
    }
}

impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
        (to_float(&s) - to_float(&t)).abs() < 0.000000001
    }
}