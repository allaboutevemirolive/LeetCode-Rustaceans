// https://leetcode.com/problems/patching-array/solutions/1432715/rust-solution/
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let n = n as u64;
        let mut it = nums.into_iter().peekable();

        std::iter::successors(Some((1, 0)), |&(x, cnt)| {
            Some(it.peek().cloned().filter(|&y| (y as u64) <= x).map_or_else(
                || (2 * x, cnt + 1),
                |y| {
                    it.next();
                    (x + y as u64, cnt)
                },
            ))
        })
        .find(|&(x, _)| x > n)
        .unwrap()
        .1
    }
}