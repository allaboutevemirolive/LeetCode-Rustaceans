// https://leetcode.com/problems/triangle/solutions/2147426/rust-dp-like-solution-with-reversal-iteration/
use std::cmp;

impl Solution {
  pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut old: Vec<i32> = triangle.last().unwrap().to_vec();
    for nums in triangle.iter().rev().skip(1) {
      let mut new = Vec::<i32>::new();
      for (j, num) in nums.iter().enumerate() {
        new.push(num + cmp::min(old[j], old[j+1]));
      }
      old = new;
    }
    old[0]
  }
}