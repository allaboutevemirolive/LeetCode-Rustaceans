// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/3602586/rust-python3-clearly-explain/
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let mut origin = arr[1];
        let mut difference = origin - arr[0];
        for index in 2..arr.len() {
            if difference != arr[index] - origin {
                return false
            }
            origin = arr[index];
        }

        return true
    }
}