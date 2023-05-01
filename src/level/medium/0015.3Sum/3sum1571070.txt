// https://leetcode.com/problems/3sum/solutions/1571070/rust/
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 { return vec![]};
        let mut nums = nums;
        nums.sort();

        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut i = 0;
        while i < nums.len()-2 {
            let target = -nums[i];  // two-sum problem: x + y = -z
            let mut x = i+1;
            let mut y = nums.len() -1;

            while x < y {
                let current = nums[x] + nums[y];
                if current < target {
                    x += 1;
                } else if current > target{
                    y -= 1;
                } else {
                    result.push(vec![-target, nums[x], nums[y]]);
                    
                    while x < y && nums[x + 1] == nums[x] {
                        x += 1;
                    }

                    while x < y && nums[y - 1] == nums[y] {
                        y -= 1;
                    }
                    
                    x += 1;
                    y -= 1;
                }
                
                while (i < nums.len()-2) && (nums[i] == nums[i+1]) {
                    i += 1;
                }
            }
            
            i += 1;
        }

        result
    }
}