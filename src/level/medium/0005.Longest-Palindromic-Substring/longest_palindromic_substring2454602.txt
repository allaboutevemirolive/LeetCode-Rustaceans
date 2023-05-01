// https://leetcode.com/problems/longest-palindromic-substring/solutions/2454602/rust-solution-15ms/
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bs = s.as_bytes();
        if bs.len() == 0 {
            return String::from("");
        }
        let mut left = 0;
        let mut right = 0;
        let mut max_len = 1;
        for i in 0..bs.len(){
            let mut l = i as i32;
            let mut r = i as i32;
            while l >= 0 && r < (bs.len() as i32) && bs[l as usize] == bs[r as usize] {
                if r-l + 1 > max_len {
                    max_len = r-l+1;
                    left = l;
                    right = r;
                }
                l -= 1;
                r += 1;
               
            }
            l = i as i32;
            r = (i+1) as i32;
            while l >= 0 && r < (bs.len() as i32) && bs[l as usize] == bs[r as usize] {
                if r-l + 1 > max_len {
                    max_len = r-l+1;
                    left = l;
                    right = r;
                }
                l -= 1;
                r += 1;
            }
        }
        return std::str::from_utf8(&bs[(left as usize)..((right+1) as usize)]).unwrap().to_string();
    }
    
}