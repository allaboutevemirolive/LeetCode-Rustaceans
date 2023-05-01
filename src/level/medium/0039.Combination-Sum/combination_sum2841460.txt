// https://leetcode.com/problems/combination-sum/solutions/2841460/rust-backtracking-0ms/
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut curr = Vec::new();
        let mut result = Vec::new();
        Self::backtracking(&candidates, target, &mut result, &mut curr);
        result
    }
    fn backtracking(
        candidates: &[i32],
        target: i32,
        result: &mut Vec<Vec<i32>>,
        curr: &mut Vec<i32>,
    ) {
        let sum = curr.iter().sum::<i32>();
        if sum == target {
            result.push(curr.to_owned());
            return;
        }
        if sum > target {
            return;
        }
        for (i, &c) in candidates.iter().enumerate() {
            curr.push(c);
            Self::backtracking(&candidates[i..], target, result, curr);
            curr.pop();
        }
    }
}