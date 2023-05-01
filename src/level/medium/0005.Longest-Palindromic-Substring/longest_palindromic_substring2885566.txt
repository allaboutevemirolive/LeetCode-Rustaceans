// https://leetcode.com/problems/longest-palindromic-substring/solutions/2885566/rust/
impl Solution {
    
    pub fn longest_palindrome(s: String) -> String {
        use std::iter::FromIterator;
        let n = s.len();
        if n == 0 {
            return "".to_string()
        }
        let s: Vec<char> = s.chars().collect();
        let mut start = 0;
        let mut end = 0;
        for i in 0..n {
            let mut right = i;
            let mut left = i;
            while right +1 < n && s[right+1] == s[left] {
                right += 1;
            }
            while left > 0 && right +1 < n && s[left-1] == s[right+1] {
                left -= 1;
                right +=1;
            }
            if right - left > end - start {
                start = left;
                end = right;
            }
        }
        String::from_iter(&s[start..=end])
    }
    
}