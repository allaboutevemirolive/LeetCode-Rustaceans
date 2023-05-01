// https://leetcode.com/problems/3sum/solutions/1742861/rust-20ms/
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(nums.len() / 3 + 1);
        for (i, &num) in nums.iter().enumerate() {
            if i > 0 && nums[i - 1] == num {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while (l < r) {
                let s = nums[l] + nums[r] + num;
                match s {
                    d if d < 0 => l += 1,
                    d if d > 0 => r -= 1,
                    _ => {
                        res.push(vec![num, nums[l], nums[r]]);
                        l += 1;
                        r -= 1;
                        while (l < r && nums[l] == nums[l - 1]) {
                            l += 1;
                        }
                        while (l < r && nums[r] == nums[r + 1]) {
                            r -= 1;
                        }
                    }
                }
            }
        }
        
        res
    }
}