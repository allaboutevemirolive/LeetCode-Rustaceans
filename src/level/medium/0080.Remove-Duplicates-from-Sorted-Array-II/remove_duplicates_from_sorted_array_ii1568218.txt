// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/1568218/rust-o-n/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut count = 0; 
        let mut j = 0; 
        let mut behind = nums[j];
        for i in 0..nums.len() {
            let front = nums[i];
            if front == behind {
                count += 1;
                if count <= 2 {
                    nums[j] = front;
                    j += 1;
                } 
            } else if front > behind {
                nums[j] = front; 
                j += 1; 
                count = 1; 
                behind = front;
            } 
        }
        
        return j as i32; 
    }
}