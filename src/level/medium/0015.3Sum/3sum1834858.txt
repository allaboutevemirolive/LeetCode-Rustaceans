// https://leetcode.com/problems/3sum/solutions/1834858/rust-n-sum/
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();

        return Self::n_sum(&nums, 3, 0, 0);
    }

    fn n_sum(nums: &Vec<i32>, n: usize, start: usize, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 2 || n > nums.len() {
            return vec![];
        }
        let mut res = vec![];

        if n == 2 {
            // two sum
            let mut lo = start;
            let mut hi = nums.len() - 1;
            while lo < hi {
                let sum = nums[lo] + nums[hi];
                let left = nums[lo];
                let right = nums[hi];
                if sum < target {
                    while lo < hi && nums[lo] == left {
                        lo += 1;
                    }
                } else if sum > target {
                    while lo < hi && nums[hi] == right {
                        hi -= 1;
                    }
                } else {
                    res.push(vec![left, right]);
                    while lo < hi && nums[lo] == left {
                        lo += 1;
                    }
                    while lo < hi && nums[hi] == right {
                        hi -= 1;
                    }
                }
            }
        } else {
            let mut lo = start;
            while lo < nums.len() {
                let left = nums[lo];
                let sub_sum = Self::n_sum(nums, n - 1, lo + 1, target - nums[lo]);

                for mut sum in sub_sum {
                    sum.push(left);
                    res.push(sum);
                }

                while lo < nums.len() && nums[lo] == left {
                    lo += 1;
                }
            }
        }

        res
    }
}