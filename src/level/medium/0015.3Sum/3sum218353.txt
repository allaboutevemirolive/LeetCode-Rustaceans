// https://leetcode.com/problems/3sum/solutions/218353/rust-32ms/
impl Solution {
    pub fn three_sum(&self, mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut found = Vec::with_capacity(nums.len() / 3);

        if nums.len() < 3 {
            return found;
        }

        nums.sort();

        let mut left;
        let mut right;
        let mut second;
        let mut third;
        let mut result;

        for (index, num) in nums.iter().enumerate() {
            if *num > 0 {
                break;
            }
            left = index + 1;
            right = nums.len() - 1;
            while left < right {
                second = nums[left];
                third = nums[right];
                result = *num + second + third;
                if result == 0 {
                    /*
                     Need to check if we don't have duplicates
                    */
                    let mut is_found = false;
                    let mut rev: i32 = found.len() as i32 - 1;
                    let mut prev;
                    while rev >= 0 {
                        prev = &found[rev as usize];
                        if prev[0] == *num {
                            if prev[1] == second && prev[2] == third {
                                is_found = true;
                                break;
                            }
                        } else {
                            //start next block
                            break;
                        }
                        rev -= 1;
                    }
                    if !is_found {
                        found.push(vec![*num, second, third]);
                    }
                    left += 1;
                    right -= 1;
                } else if result > 0 {
                    right -= 1;
                } else if result < 0 {
                    left += 1;
                }
            }
        }

        found
    }
}