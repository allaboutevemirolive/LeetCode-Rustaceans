// https://leetcode.com/problems/sliding-window-median/solutions/724965/rust-runtime-12ms-faster-than-100-00-use-sorted-vector-and-binary-search/
struct Solution;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        fn median(window: &Vec<i32>, k: i32) -> f64 {
            if k % 2 == 1 {
                window[((k - 1) / 2) as usize] as f64
            } else {
                (window[(k / 2) as usize] as f64 + window[(k / 2 - 1) as usize] as f64) / 2.0
            }
        }

        let len = nums.len();
        let cycle = len as i32 - k + 1;
        if cycle < 1 {
            return vec![];
        }
        if k < 2 {
            return nums.iter().map(|&x| x as f64).collect();
        }
        let mut ans = vec![0.0; cycle as usize];
        let mut window = Vec::<i32>::new();
        for i in 0..k {
            let pos = match window.binary_search(&nums[i as usize]) {
                Ok(i) => i,
                Err(i) => i,
            };
            window.insert(pos, nums[i as usize]);
        }
        ans[0] = median(&window, k);
        for i in 1..cycle {
            match window.binary_search(&nums[(i - 1) as usize]) {
                Ok(i) => window.remove(i),
                _ => unreachable!(),
            };
            let pos = match window.binary_search(&nums[(k + i - 1) as usize]) {
                Ok(i) => i,
                Err(i) => i,
            };
            window.insert(pos, nums[(k + i - 1) as usize]);
            ans[i as usize] = median(&window, k);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_median_sliding_window() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        assert_eq!(
            Solution::median_sliding_window(nums, 3),
            vec![1.0, -1.0, -1.0, 3.0, 5.0, 6.0]
        )
    }
}
