// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/2171361/rust-solution-linear-time-complexity-two-pointers/
impl Solution {
 pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
  let mut ans = i32::MAX;
  let s = nums.len();
  let (mut start, mut end) = (0, 0);

  let mut cur = 0;
  let mut counter = 0;
  while end < s {
   cur += nums[end];
   counter += 1;
   while cur >= target {
    if cur >= target {
     ans = std::cmp::min(counter, ans);
    }
    cur -= nums[start];
    counter -= 1;
    start += 1;
   }
   end += 1;
  }
  if ans == i32::MAX {
   return 0;
  } else {
   return ans;
  }
 }
}