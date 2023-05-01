// https://leetcode.com/problems/single-number-ii/solutions/1562761/rust-bitwise/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut twos = 0;
        
        for n in nums {
            ones = (ones ^ n) & !twos;
            twos = (twos ^ n) & !ones;
        }
        
        ones
    }
}```