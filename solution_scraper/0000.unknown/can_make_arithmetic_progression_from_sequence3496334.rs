// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/3496334/rust-solution-using-slinding-window/
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let d = arr[1] - arr[0];

        if arr.windows(2).any(|x| x[1] - x[0] != d) {
            return false;
        }

        true
    }
}