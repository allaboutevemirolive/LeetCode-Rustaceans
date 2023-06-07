// https://leetcode.com/problems/special-binary-string/solutions/493790/rust-0ms-2mb-100/
pub fn make_largest_special(s: String) -> String {
    fn helper(s: &str) -> String {
        let (mut count, mut i) = (0, 0);
        let mut res = Vec::<String>::new();
        for j in 0..s.len() {
            if s[j..].chars().next().unwrap() == '1' { count += 1; } else { count -= 1; }
            if count == 0 {
                let mut s1 = String::new();
                s1.push('1');
                s1.push_str(helper(&s[i + 1..j]).as_str());
                s1.push('0');
                res.push(s1);
                i = j + 1;
            }
        }
        res.sort_by(|a, b| { b.cmp(a) });
        res.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&x[..]);
            acc
        })
    }
    helper(&s)
}
