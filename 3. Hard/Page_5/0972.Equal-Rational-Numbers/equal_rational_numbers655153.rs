// https://leetcode.com/problems/equal-rational-numbers/solutions/655153/rust-string-to-float-64-0ms/
impl Solution {
    pub fn unfold(s: String) -> f64 {
        if !s.contains("(") {
            return s.parse::<f64>().unwrap();
        }
        let i = s.find('(').unwrap();
        let rep = &s[i + 1..s.len() - 1];
        let mut base = (&s[..i]).to_string();
        for i in 1..20 {
            base += rep;
        }
        base.parse::<f64>().unwrap()
    }

    pub fn is_rational_equal(s: String, t: String) -> bool {
        Self::unfold(s) == Self::unfold(t)
    }
}