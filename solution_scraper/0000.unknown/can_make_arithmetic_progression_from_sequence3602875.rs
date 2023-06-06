// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/3602875/rust/
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let t = arr[1] - arr[0];

        for i in 2..arr.len() {
            if arr[i] - arr[i - 1] != t {
                return false;
            }
        }
        true
    }
}