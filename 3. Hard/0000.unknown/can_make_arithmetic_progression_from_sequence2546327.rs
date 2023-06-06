// https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/solutions/2546327/rust-100-faster-sorting/
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {   
        arr.sort();
        
        let mut diffs = arr.windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<i32>>();
        
        if diffs.is_empty() {
            return true;
        }
        
        let first = diffs[0];
        diffs.iter()
            .all(|a| *a == first)
    }
}