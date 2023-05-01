// https://leetcode.com/problems/find-peak-element/solutions/968733/rust-ez-o-n/
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        
        nums.iter().zip(nums.iter().skip(1)).take_while(|(left, right)|left < right).count() as i32
    }
}