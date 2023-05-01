// https://leetcode.com/problems/longest-palindromic-substring/solutions/984643/rust-solution/
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        let mut res = (0, 0);
        // iterate 1..n, find all max odd num palindrome
        for i in 1..(n - 1) {
            let mut offset = 0;
            while i >= offset + 1
                && i + offset + 1 < n
                && chars[i - offset - 1] == chars[i + offset + 1]
            {
                offset += 1;
            }
            let l = i - offset;
            let r = i + offset;
            if r - l > res.1 - res.0 {
                res = (l, r);
            }
        }
        // iterate 0..n, find all even num palidrome
        for mut l in 0..(n - 1) {
            let mut r = l + 1;
            if chars[l] != chars[r] {
                continue;
            }
            while l > 0 && r < n - 1 && chars[l - 1] == chars[r + 1] {
                l -= 1;
                r += 1;
            }
            if r - l > res.1 - res.0 {
                res = (l, r);
            }
        }
        chars[res.0..=res.1].iter().collect()
    }
}