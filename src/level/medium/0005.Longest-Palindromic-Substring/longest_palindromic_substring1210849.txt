// https://leetcode.com/problems/longest-palindromic-substring/solutions/1210849/rust/
impl Solution {
    // Use of `.windows()` for a slice taken from user: stevensonmt's
    // solution on leetcode here: https://leetcode.com/problems/longest-palindromic-substring/discuss/467258/Straight-forward-solution-with-slice-windows
    // Other parts of the solution are my own solution
    pub fn longest_palindrome(s: String) -> String {
        fn is_palidrone(s: &[u8]) -> bool {
            // Iterate left to right along with iterating from right to left, 
            // make sure each spot is the same.
            // Returns false once the left is not equal to the right
            s.iter().zip(s.iter().rev()).all(|(l, r)| l == r)
        }
        
        for size in (1..=s.len()).rev() {
            match s.as_bytes()
                .windows(size)
                .find(|substr| is_palidrone(substr)) {
                Some(pal) => return String::from_utf8(pal.to_vec()).unwrap(),
                None => continue,
            }
        }
        // No palidrone found
        String::from("")
    }

}