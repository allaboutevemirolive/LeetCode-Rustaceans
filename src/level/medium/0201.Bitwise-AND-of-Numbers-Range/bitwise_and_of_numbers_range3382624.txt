// https://leetcode.com/problems/bitwise-and-of-numbers-range/solutions/3382624/rust-solution/
impl Solution {
  pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
    if left < right {
      std::mem::swap(&mut left, &mut right);
    }
    for i in (0..32).rev() {
      let v = 1 << i;
      if left & v == v {
        let mut result = 0;
        for j in (0..=i).rev() {
          let v2 = 1 << j;
          let l = left & v2 == v2;
          let r = right & v2 == v2;
          
          if l != r {
            break
          }

          if left & v2 == v2 {
            result += v2;
          }
        }
        return result
      }
    }
    0
  }
}