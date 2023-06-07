// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/2025917/rust-sort-windows/
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        arr[1..].windows(2).all(|w| w[1] - w[0] == arr[1] - arr[0])
    }
}