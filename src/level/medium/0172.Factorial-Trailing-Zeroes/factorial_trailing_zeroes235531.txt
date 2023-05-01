// https://leetcode.com/problems/factorial-trailing-zeroes/solutions/235531/my-rust-solution-in-0ms/
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        
        fn helper(n: &i32) -> i32 {
            if *n == 0 {
                0
            } else {
                n/5 + helper(&(n/5))
            }
        }
        
        let ret = helper(&n);
        ret
    }
}