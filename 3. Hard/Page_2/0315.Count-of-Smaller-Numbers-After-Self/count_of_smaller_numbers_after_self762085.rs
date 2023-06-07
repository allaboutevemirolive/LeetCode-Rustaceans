// https://leetcode.com/problems/count-of-smaller-numbers-after-self/solutions/762085/rust-translated-merge-sort/
impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        #[inline]
        fn merge(nums: &[i32], indices: &mut [usize], ans: &mut [i32], start: i32, end: i32) {
            let mid = (start + end) / 2;
            let mut left_idx = start;
            let mut right_idx = mid + 1;
            let mut right_count = 0;
            let mut new_indices = vec![0usize; (end - start + 1) as usize];

            let mut sort_idx = 0;
            while left_idx <= mid && right_idx <= end {
                if nums[indices[right_idx as usize]] < nums[indices[left_idx as usize]] {
                    new_indices[sort_idx] = indices[right_idx as usize];
                    right_count += 1;
                    right_idx += 1;
                } else {
                    new_indices[sort_idx] = indices[left_idx as usize];
                    ans[indices[left_idx as usize]] += right_count;
                    left_idx += 1;
                }
                sort_idx += 1;
            }
            while left_idx <= mid {
                new_indices[sort_idx] = indices[left_idx as usize];
                ans[indices[left_idx as usize]] += right_count;
                left_idx += 1;
                sort_idx += 1;
            }
            while right_idx <= end {
                new_indices[sort_idx] = indices[right_idx as usize];
                sort_idx += 1;
                right_idx += 1;
            }
            for i in start..=end {
                indices[i as usize] = new_indices[(i - start) as usize];
            }
        }

        fn merge_sort(nums: &[i32], indices: &mut [usize], ans: &mut [i32], start: i32, end: i32) {
            if end <= start {
                return;
            }
            let mid = (start + end) / 2;
            merge_sort(nums, indices, ans, start, mid);
            merge_sort(nums, indices, ans, mid + 1, end);
            merge(nums, indices, ans, start, end)
        }

        let mut indices = (0..nums.len()).collect::<Vec<usize>>();
        let mut ans = vec![0; nums.len()];
        merge_sort(&nums, &mut indices, &mut ans, 0, nums.len() as i32 - 1);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_smaller() {
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0])
    }
}