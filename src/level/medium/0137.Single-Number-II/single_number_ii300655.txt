// https://leetcode.com/problems/single-number-ii/solutions/300655/rust-solution/
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let mut sum = 0;
            for j in 0..nums.len() {
                sum += (nums[j] >> i) & 1;
            }
            res |= (sum % 3) << i;
        }
        res        
    }
}