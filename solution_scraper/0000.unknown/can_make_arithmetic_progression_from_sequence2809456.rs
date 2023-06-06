// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/2809456/leetcode-the-hard-way-rust-sort-windows/
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
		// sort the arr first
        arr.sort();
		// then we focus on a window of size of 3
        for w in arr.windows(3) {
			// check if the difference between w[0] and w[1] and that between w[1] and w[2]  is same
			// if not, then we cannot make an AP
            if w[2] - w[1] != w[1] - w[0] {
                return false;
            }
        }
        true
    }
}