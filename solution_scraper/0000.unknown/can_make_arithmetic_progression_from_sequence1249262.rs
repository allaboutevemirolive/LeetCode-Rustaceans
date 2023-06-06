// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/1249262/rust-solution-with-sorting/
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();

        let delta = arr[1] - arr[0];

        arr.iter()
            .enumerate()
            .skip(1)
            .all(|(ind, val)| val - arr[ind - 1] == delta)
    }
}