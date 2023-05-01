// https://leetcode.com/problems/longest-consecutive-sequence/solutions/3285283/rust-sort/
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // clone the vector
        let mut sorted = nums.clone();

        // sort the vector
        sorted.sort();

        // initialize the max length and the current length
        let mut max = 0;
        let mut count = 0;

        for i in 0..sorted.len() {
            if i == 0 || sorted[i] == sorted[i - 1] + 1 {
                // if the current number is the first number or
                // the current number is the next number of the previous number
                // then increase the current length by 1

                count += 1;
            } else if sorted[i] == sorted[i - 1] {
                // if the current number is the same as the previous number
                // do nothing

                continue;
            } else {
                // otherwise, the current length becomes 1
                
                count = 1;
            }
            // update the max length
            max = std::cmp::max(max, count);
        }

        // return the max length
        max
    }
}