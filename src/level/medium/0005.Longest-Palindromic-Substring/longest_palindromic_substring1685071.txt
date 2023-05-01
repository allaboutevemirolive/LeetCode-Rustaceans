// https://leetcode.com/problems/longest-palindromic-substring/solutions/1685071/3-solutions-using-rust/
// 1. rolling hash
use std::str;
static M: i64 = 1e9 as i64 + 7;
static p: i64 = 131;
// 2. DP
static NOTSET: (i64, i64) = (-1,-1);

impl Solution {
    // there can be at least 3 quadratic solutions
    // 1. rolling hash ~ 533ms
    fn getCode(c: u8) -> i64 { return c as i64; }
    pub fn longest_palindrome(str: String) -> String {
        let s = str.as_bytes();
        let n: usize = s.len();
        if n == 0 {
            return str;
        }
        let mut bh: Vec<i64> = vec![0; n+1];
        let mut pow: Vec<i64> = vec![0; n];
        let mut pp: i64 = 1;
        for i in (0..n).rev() {
            bh[i] = (bh[i+1] + (Self::getCode(s[i]) * pp)%M)%M;
            pow[n - 1 - i] = pp;
            pp = (pp * p)%M;
        }
        
        let mut res: &[u8] = &s[0..1];     
        for i in 0..n {
            let mut fh: i64 = 0;
            pp = 1;
            for j in i..n {
                fh = (fh + (Self::getCode(s[j])*pp)%M)%M;
                let curFH = (fh * pow[n - 1 - j])%M;
                let curBH = (bh[i] - bh[j+1] + M)%M;
                pp = (pp * p)%M;
                if curBH == curFH {
                    if res.len() < (j - i + 1) {
                        res = &s[i .. j + 1];
                    }
                }
            }
        }
        str::from_utf8(res).unwrap().to_string()
    }
    
    // 2. DP ~ 206ms
    pub fn longest_palindrome(str: String) -> String {
        let s = str.as_bytes();
        let n: usize = s.len();
        if n == 0 { 
            return "".to_string();
        }
        let mut dp = vec![vec![false; n]; n];
        let mut res: (i64, i64) = NOTSET;
        for j in 0..n {
            for i in (0..=j).rev() {
                let diff = (j - i) as i64;
                if s[i] == s[j] {
                    dp[i][j] = (diff < 2) || dp[i+1][j-1];
                }
                if dp[i][j] && (res == NOTSET || res.1 - res.0 < diff) {
                    res = (i as i64, j as i64);
                }
            }
        }
        return str[res.0 as usize .. (res.1 + 1) as usize].to_string();
    }
    
    // 3. Expand from the middle of each element ~13ms
    pub fn longest_palindrome(str: String) -> String {
        let s = str.as_bytes();
        let n: i64 = s.len() as i64;
        if n == 0 {
            return str;
        }
        let mut res: &[u8] = &s[0..1];
         
        // check for palindromes with center in m with odd length
        for m in 0..n {
            let mut l: i64 = m as i64;
            let mut r: i64 = m as i64;
            while l >= 0 && r < n && s[l as usize] == s[r as usize] {
                l -= 1;
                r += 1;
            }
            l += 1;
            r -= 1;
            if (res.len() as i64) < (r + 1 - l) {
                res = &s[l as usize .. (r + 1) as usize];
            }
        }
        
        // check for palindromes with center in m with even length
        for m in 0..(n-1) {
            let mut l = m as i64;
            let mut r = (m+1) as i64;
            while l >= 0 && r < n && s[l as usize] == s[r as usize] {
                l -=1; 
                r += 1;
            }
            l += 1;
            r -= 1;
            if (res.len() as i64) < r - l + 1 {
                res = &s[l as usize .. (r + 1) as usize];
            }
         }
        
        str::from_utf8(res).unwrap().to_string()
    }
    
}