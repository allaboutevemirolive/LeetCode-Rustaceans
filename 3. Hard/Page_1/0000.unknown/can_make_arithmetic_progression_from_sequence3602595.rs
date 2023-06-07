// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/3602595/rust-sort-solution/
impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;

        arr.sort();

        for i in 0..arr.len() {
            if i >= 2 && arr[i] - arr[i - 1] != arr[i - 1] - arr[i - 2] {
                return false;
            }
        }

        true
    }
}