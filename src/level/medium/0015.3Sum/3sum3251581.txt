// https://leetcode.com/problems/3sum/solutions/3251581/simple-rust-solution/
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = std::collections::HashSet::new();
        nums.sort();
        for (i, &num) in nums.iter().enumerate() {
            if i > 0 && num == nums[i-1] {continue}
            let mut l = i+1;
            let mut r = nums.len() - 1;
            while l < r {
                let s = num + nums[l] + nums[r];
                if s > 0 {
                    r -= 1;
                } else if s < 0 {
                    l += 1;
                } else {
                    ans.insert(vec![num, nums[l], nums[r]]);
                    l += 1;
                }
            }
        }
        let res = ans.into_iter().collect::<Vec<Vec<i32>>>();
        res
    }
}