// https://leetcode.com/problems/sliding-window-median/solutions/1341219/rust-binary-search-vec-heap-beats-100-mem-and-speed/
impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut window = Vec::<i32>::new();

        let mut res: Vec<f64> = vec![];
        // insert all, and keep them in order
        for (ix, i) in nums.iter().enumerate() {
            // find where to insert
            let idx = match window.binary_search(i) { Ok(x) => x, Err(x) => x };
            // add new elem
            window.insert(idx, *i);

            // remove oldest elem
            if window.len() > k as usize {
                let oldest = nums[ix - k as usize];
                window.remove(window.binary_search(&oldest).unwrap());
            }
            // get median
            // @warn index calculation!
            if window.len() == k as usize {
                let mid = k as usize / 2;
                let median = if k % 2 == 1 {
                    window[mid] as f64
                } else {
                    window[mid - 1] as f64 * 0.5 + window[mid] as f64 * 0.5
                };
                res.push(median);
            }
        }
        res
    }
}