// https://leetcode.com/problems/permutations/solutions/2997805/rust-backtracking/
impl Solution {
    fn construct_permutation(nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>) {
        if cur.len() == nums.len() {
            ans.push(cur.to_vec());
        } else {
            for num in nums.iter() {
                if !cur.contains(&num) {
                    cur.push(*num);
                    Solution::construct_permutation(&nums, ans, cur);
                    cur.pop();
                }
            }
        }
    }
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut cur: Vec<i32> = Vec::new();
        Solution::construct_permutation(&nums, &mut ans, &mut cur);
        ans
    }
}

