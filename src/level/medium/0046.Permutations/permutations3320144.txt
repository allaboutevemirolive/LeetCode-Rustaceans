// https://leetcode.com/problems/permutations/solutions/3320144/rust-backtracking-solution/
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(nums.len());
        Self::permute_new(vec![], &nums, &mut res);
        res
    }
    pub fn permute_new(have: Vec<i32>, left: &Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if left.is_empty() {
            return  res.push(have);
        }
        for (i, &val) in left.iter().enumerate() {
            let mut new = have.clone();
            new.push(val);
            Self::permute_new(new, &[&left[..i], &left[i+1..]].concat(), res);
        }
    }
}