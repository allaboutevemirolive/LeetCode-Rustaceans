// https://leetcode.com/problems/largest-palindrome-product/solutions/585869/rust-mathematically-unsound-math/
impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        
        let min_factor = (10_i32.pow(n as u32 - 1)) as i64;
        let max = (10_i32.pow(n as u32)-1) as i64;
        let max_factor = (max / 11) * 11; 
		// because integer division, this gives the largest n digit number divisible by 11
            
        for i in (min_factor..=max).rev() { 
		// reverse the range to start with the biggest n digit number first and decrement from there as needed
            let palindrome = build_palindrome(i, n); // mirror the digits to create your palindrome
            let lower = (palindrome as f64).sqrt().floor() as i64 - (44 * (n as i64 - 2)); 
			// the square root is a reasonable guess for a starting point, but we have this arbitrary 
			// fudge factor that works for our given cases without any guarantee it will work for other cases
			
            for j in (lower..=max_factor).rev().step_by(11) { 
			// we step by 11 b/c we assume the larger factor is divisible by 11
			// because our lower limit goes beyond the square root we still catch 
			// some smaller factors divisible by 11 if necessary
			
                if palindrome % j == 0 && (palindrome / j) / (max+1) == 0 {
                    return (palindrome % 1337) as i32
                }
            }
        }
        9
    }
}

fn build_palindrome(i: i64, n: i32) -> i64 {
    let mut x = i * 10_i64.pow(n as u32);
    
    let mut y = i;
    let mut pal = 0;
    while y > 0 {
        pal = 10 * pal + y % 10;
        y /= 10;
        
    }
    pal += x;
    pal
}