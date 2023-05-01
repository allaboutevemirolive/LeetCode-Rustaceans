// https://leetcode.com/problems/longest-palindromic-substring/solutions/2840274/rust-57ms/
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut answer: String = String::new();
        let mut s: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            for j in i + answer.len()..s.len() {
                // substring length > answer length
                if s[i..=j].len() > answer.len() &&
                   // substring == reversed(substring)
                   s[i..=j].iter().rev().eq(s[i..=j].iter())
                {
                    answer = s[i..=j].iter().collect();
                }
            }
        }
        answer
    }
}