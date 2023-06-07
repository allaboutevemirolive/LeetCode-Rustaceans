// https://leetcode.com/problems/longest-duplicate-substring/solutions/1548090/rust-solution/
impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let mut max_len = 0;
        let mut str_start = 0;
        
        // case of repeated characters
        let str = s.as_bytes();
        if str.iter().all(|&value| value == str[0]) {
            return s[1..s.len()].to_string();
        }
        
		let mut start = 0;
        for end in 1..s.len(){
            if max_len >= (end - start) {
                break;
            }
            
            let str_temp = &s[start..end];
            
            if s[(start+1)..s.len()].contains(str_temp) && str_temp.len() > max_len {
                str_start = start;
                max_len = str_temp.len();
            } else {
                start+=1;
            }
        }
        
        return s[str_start..(str_start + max_len)].to_string();
    }
}