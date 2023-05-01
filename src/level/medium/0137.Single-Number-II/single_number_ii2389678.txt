// https://leetcode.com/problems/single-number-ii/solutions/2389678/rust-concise-solution-with-ternary-logic-explained/
impl Solution {
  pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter()
      .fold([0, 0], |[a, b], x|
        [ a ^ x & !b,
          a & x | b & !x ]
      )[0]
  }
}