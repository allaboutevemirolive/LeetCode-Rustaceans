// https://leetcode.com/problems/combination-sum/solutions/1813493/rust-backtracking/
impl Solution {
    fn combi(res: &mut Vec<Vec<i32>>, buffer: &mut Vec<i32>, candidates: &[i32], target: &i32) {
        buffer.push(candidates[0]);

        if buffer.iter().sum::<i32>()==*target {
            res.push(buffer.clone());
        } else if buffer.iter().sum::<i32>()<*target {
            for i in 0..candidates.len() {
                Solution::combi(res, buffer, &candidates[i..], target);
            }
        }

        buffer.pop();
    }
    
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let mut buffer: Vec<i32>;
        for i in 0..candidates.len() {
            buffer = vec![];
            Solution::combi(&mut res, &mut buffer, &candidates[i..], &target);
        }
        res
    }
}