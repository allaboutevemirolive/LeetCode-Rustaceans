// https://leetcode.com/problems/longest-happy-prefix/solutions/628067/rust-simple-solution-0ms/
impl Solution {
    pub fn longest_prefix(s: String) -> String {
        let mut pre_len = 0; 
        let mut lps = vec![0; s.len()];
        let cs: Vec<char> = s.chars().collect(); 
        for i in (1..s.len()) {
            while pre_len > 0 && cs[i] != cs[pre_len] {
                pre_len = lps[pre_len - 1] as usize; 
            }
            if cs[i] == cs[pre_len] {
                pre_len += 1; 
                lps[i] = pre_len; 
            }
        }
        s[..*lps.last().unwrap()].to_string()
    }
}