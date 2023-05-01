// https://leetcode.com/problems/permutations/solutions/3233015/rust/
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut data = Vec::new();

        Self::backtrack(&data, &mut ans, &nums);

        ans
    }

    fn backtrack(data: &Vec<i32>, ans: &mut Vec<Vec<i32>>, nums: &Vec<i32>) {
        if data.len() == nums.len() {
            ans.push(data.clone());
        } else {
            for i in 0..nums.len() {
                if (!data.contains(&nums[i])) {
                    let mut temp = data.clone();
                    temp.push(nums[i]);
                    Self::backtrack(&temp, ans, nums);
                }
            }
        }
    }
}