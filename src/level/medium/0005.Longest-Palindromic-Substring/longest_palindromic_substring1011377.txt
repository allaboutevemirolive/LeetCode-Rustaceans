// https://leetcode.com/problems/longest-palindromic-substring/solutions/1011377/rust-slice-solution/
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let (mut s, mut max) = (s.chars().collect::<Vec<char>>(), vec![]);
        fn find_max(s: &Vec<char>, max: Vec<char>, i: usize, j: usize) -> Vec<char> {
            let (mut i, mut j) = (i, j);
            let mut sub: &[char] = &[];
            while i != std::usize::MAX && j < s.len() && s[i] == s[j] {
                sub = &s[i..j+1];
                i -= 1;
                j += 1;
            }
            if sub.len() > max.len() {
                return sub.to_vec()
            }
            max.to_vec()
        }
        for i in 0..s.len() {
            max = find_max(&s, max, i, i);
            max = find_max(&s, max, i, i+1);
        }
        max.into_iter().collect()
    }
}