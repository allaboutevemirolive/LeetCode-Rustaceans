// https://leetcode.com/problems/3sum/solutions/3221678/rust-solution/
use std::collections::*;

fn helper(a:i32) -> usize {
  (a + 10i32.pow(5)) as usize
}

impl Solution {
  pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let n = nums.len();
    let mut memo = vec![0;2*10usize.pow(5) + 100];
    for i in 0..n {
      memo[helper(nums[i])] += 1;
    }

    let mut result = HashSet::new();
    for i in 0..n {
      let v1 = nums[i];
      memo[helper(nums[i])] -= 1;
      for j in 0..n {
        if i == j { continue }
        let v2 = nums[j];
        memo[helper(nums[j])] -= 1;
        let v3 = -1 * (v1+v2);        
        if v3 >= -10i32.pow(5) && v3 <= 10i32.pow(5) && memo[helper(v3)] > 0 {
          let mut temp = vec![v1,v2,v3];
          temp.sort();
          result.insert(temp);
        }

        memo[helper(nums[j])] += 1;
      }
      memo[helper(nums[i])] += 1;
    }
    result.into_iter().collect::<Vec<Vec<i32>>>()
  }
}
