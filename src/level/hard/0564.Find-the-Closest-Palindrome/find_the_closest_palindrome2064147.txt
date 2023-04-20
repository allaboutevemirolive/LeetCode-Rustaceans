// https://leetcode.com/problems/find-the-closest-palindrome/solutions/2064147/rust-speed-memory-better-than-100/

use std::cmp::{min, max};

impl Solution {    
    pub fn palindromize_right(n: String) -> String {
        let l = n.len();
        let n = n.clone();
        let mut n : Vec<char> = n.chars().collect();
        for x in 0..l / 2 {
            n[l-1-x] = n[x];
        }
        n.into_iter().collect()
    }
    
    pub fn nearest_palindromic(n: String) -> String {
        if n.len() == 1 {
            return (n.parse::<i32>().unwrap() - 1).to_string();
        }
        
        let mut palindromes = Vec::new();
        
        let og_n_as_number = n.parse::<i64>().unwrap();
        
        let mut n_as_number : i64 = og_n_as_number.clone().into();
        n_as_number += i64::pow(10, n.len() as u32 / 2);
        
        let mut n_subbed : i64 = og_n_as_number.clone().into();
        n_subbed -= i64::pow(10, n.len() as u32 / 2);

        palindromes.push(Solution::palindromize_right(n.clone()));
        palindromes.push(Solution::palindromize_right(n_subbed.to_string()));
        palindromes.push(Solution::palindromize_right(n_as_number.to_string()));
        palindromes.push((i64::pow(10, n.len() as u32 + 1)).to_string());
        palindromes.push((i64::pow(10, n.len() as u32 - 1) - 1).to_string());
        
        let mut palindromes : Vec<i64> = palindromes.iter().map(|p| p.parse::<i64>().unwrap()).collect();
        
        let mut smallest_diff = i64::MAX;
        let mut smallest_palin = i64::MAX;
        
        palindromes.sort();
        palindromes.reverse();
        
        for palindrome in palindromes {
            if palindrome == og_n_as_number { continue }
            
            let current_diff = i64::abs(og_n_as_number - palindrome);
            
            if current_diff <= smallest_diff {
                smallest_diff = current_diff;
                smallest_palin = palindrome;
            }
        }
        
        smallest_palin.to_string()
    }
}
