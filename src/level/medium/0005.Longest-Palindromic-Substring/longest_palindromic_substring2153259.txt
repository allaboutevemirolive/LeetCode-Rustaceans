// https://leetcode.com/problems/longest-palindromic-substring/solutions/2153259/rust-dp/
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn helper<'a>(mut l:i64, mut r:i64, sb:&'a [u8], cur: &'a [u8]) -> &'a [u8]{
            let mut res: &[u8] = cur;
            let n = sb.len() as i64;
            while l>=0 && r<n && sb[l as usize] == sb[r as usize]{
                l -= 1;
                r += 1;
            }
            l += 1;
            r -= 1;
            if (r - l + 1)  > cur.len() as i64{
                res = &sb[l as usize..(r+1) as usize];
            }
            res
        }
        
        let sb = s.as_bytes();
        let n = sb.len() as i64;
        let mut res: &[u8] = &sb[0..1];
        for i in 0..n{
            res = helper(i, i, sb, res);
            res = helper(i, i+1, sb, res);
        }
        std::str::from_utf8(res).unwrap().to_string()
    }
}