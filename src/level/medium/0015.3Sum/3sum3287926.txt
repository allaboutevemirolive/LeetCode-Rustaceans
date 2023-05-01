// https://leetcode.com/problems/3sum/solutions/3287926/rust-2-approaches/
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn approach_with_set(nums: Vec<i32>) -> Vec<Vec<i32>> {
            use std::collections::*;
            let mut set = HashSet::new();
            let mut ans = HashSet::new();
            for i in 0..nums.len() - 1 {
                let a = nums[i];
                set.clear();
                for j in i + 1..nums.len() {
                    let b = nums[j];
                    if set.contains(&-(a + b)) {
                        let c = *set.get(&-(a + b)).unwrap();
                        let mut triplet = vec![a, b, c];
                        triplet.sort();
                        ans.insert(triplet);
                    }
                    set.insert(b);
                }
            }
            ans.into_iter().collect()
        }

        fn approach_with_two_pointers(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
            use std::cmp::*;
            nums.sort();
            let mut ans = vec![];
            for i in 0..nums.len() - 2 {
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }
                let mut left = i + 1;
                let mut right = nums.len() - 1;
                while left < right {
                    let sum = nums[i] + nums[left] + nums[right];
                    match sum.cmp(&0) {
                        Ordering::Greater => {
                            right -= 1;
                        }
                        Ordering::Less => {
                            left += 1;
                        }
                        _ => {
                            ans.push(vec![nums[i], nums[left], nums[right]]);
                            while left < right && nums[left] == nums[left + 1] {
                                left += 1;
                            }
                            while left < right && nums[right] == nums[right - 1] {
                                right -= 1;
                            }
                            left += 1;
                            right -= 1;
                        }
                    }
                }
            }
            ans
        }

        approach_with_two_pointers(nums)
    }
}