// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/solutions/1918292/rust-solution/
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i : usize = 0;
        let mut j : usize = numbers.len() - 1;
        
        while (i <= j) {
            let tot =  numbers[i] + numbers[j];
            if (tot == target) {
                return vec![i as i32 + 1 ,j as i32 + 1];
            }
            else if (tot < target) {
                i += 1;
            }
            else {
                j -= 1;
            }
        }
        return vec![-1, -1];
    }
}