// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/3189432/rust-with-binary-search/
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        for i in 1..nums.len() {
            if nums[i] < nums[i-1] {
                match nums[..i].binary_search(&target) {
                    Ok(res) => {return res as i32},
                    Err(_) => {
                        match nums[i..].binary_search(&target) {
                            Ok(res) => {return (i + res) as i32},
                            Err(_) => {return -1}
                        }        
                    }
                }
            }
        }
        match nums.binary_search(&target) {
            Ok(res) => {return res as i32},
            Err(_) => {return -1}
        }
    }
}