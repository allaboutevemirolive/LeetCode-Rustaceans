// https://leetcode.com/problems/special-binary-string/solutions/571567/rust-solution-0ms/
impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut res = vec![];
        let (mut i, mut cnt) = (0_usize, 0_i32);
        for (j, c) in s.chars().enumerate(){
            cnt += if c == '1' { 1 } else { - 1 }; 
            if cnt == 0 {
                res.push("1".to_string() + &Solution::make_largest_special(s[i+1..j].to_string()) + &"0");
                i = j + 1;
            }
        }
        res.sort_unstable_by(|a, b| b.cmp(&a));
        res.join("")
    }
}