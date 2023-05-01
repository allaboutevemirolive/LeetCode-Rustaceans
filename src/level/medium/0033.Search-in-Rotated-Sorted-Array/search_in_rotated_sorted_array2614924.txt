// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/2614924/rust-solution/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left:i32 = 0; 
        let mut right:i32 = nums.len() as i32 -1;
        let mut pivot:i32 = 0; 
        
        while left<right{
            pivot = left+(right-left)/2; 
            if nums[pivot as usize] > nums[right as usize]{
                left = pivot+1;
            }
            else{
                right = pivot;
            }
        }
        
        pivot = left;
        left = 0; 
        right = nums.len() as i32 -1;
        
        if target >= nums[pivot as usize] && target <= nums[right as usize] {
            left = pivot;
        }
        else{
            right = pivot;
        }
        
        while left<=right{
            pivot = left+(right-left)/2;
            if nums[pivot as usize] == target {
                return pivot;
            }
            if target < nums[pivot as usize]{
                right = pivot-1;
            }
            else{
                left = pivot+1;
            }
        }
        return -1;
        
    }
}