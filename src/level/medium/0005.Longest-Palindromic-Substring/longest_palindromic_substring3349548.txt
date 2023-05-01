// https://leetcode.com/problems/longest-palindromic-substring/solutions/3349548/rust-0ms/
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let n = s.len();
        let mut beg = 0;
        let mut end = 0;
        for i in 0..n {
            let mut l = i;
            let mut r = i;
            while r + 1 < n && s[r + 1] == s[l] {
                r += 1;
            }
            while l > 0 && r + 1 < n && s[l - 1] == s[r + 1] {
                l -= 1;
                r += 1;
            }
            if r - l > end - beg {
                beg = l;
                end = r;
            }
        }
        String::from_utf8(s[beg..=end].to_vec()).unwrap()
    }
}