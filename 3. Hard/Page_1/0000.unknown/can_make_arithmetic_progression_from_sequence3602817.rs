// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/3602817/intuitive-solution-in-rust/
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();
        let diff = arr[1] - arr[0];
        if arr.len() == 2 {
            true
        } else {
            for (i, a) in arr.iter().skip(2).enumerate() {
                let mut j = i + 2; // index incorrectly starts from 0
                if a - arr[j - 1] != diff {
                    return false;
                }
            }
            true
        }
    }
}