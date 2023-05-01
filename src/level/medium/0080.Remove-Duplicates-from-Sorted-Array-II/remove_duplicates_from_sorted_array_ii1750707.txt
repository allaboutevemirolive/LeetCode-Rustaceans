// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solutions/1750707/rust-method-1/
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        match nums.len() {
            0 | 1 => nums.len() as i32,
            _ => {
                let mut i:usize = 2;
                loop {// can use for i in 2..a.len(), because a.len() is a fixed number, we will have to check i < a.len() inside for loop
                    if i < nums.len() {
                        if nums[i-2] == nums[i] {
                            nums.remove(i);
							// shouldn't increase index i, because after removing ith element from array, i+1th element will take ith element place in the new vector
                        } else {
                            i+=1;
                        }
                    }
                    if i == nums.len() {
                        break;
                    }
                }
                return nums.len() as i32;
            }
        }
    }
}