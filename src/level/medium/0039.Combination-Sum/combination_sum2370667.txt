// https://leetcode.com/problems/combination-sum/solutions/2370667/rust-backtracking-0ms/
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut perms = Vec::new();
        let mut temp  = Vec::new();
        Self::perm(&mut perms, &mut temp, &candidates, target, 0, 0);
        perms
    }
    
    pub fn perm(res: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, nums: &Vec<i32>, target: i32, start: usize, mut sum: i32) {
        if sum > target {
            return;   
        }
        if sum == target{
            res.push(temp.to_vec());
        }
        
        for n in start..nums.len() {
            temp.push(nums[n]);
            sum += nums[n];
            Self::perm(res, temp, nums, target, n, sum);
            temp.pop();
            sum -= nums[n];
        }
    }
}