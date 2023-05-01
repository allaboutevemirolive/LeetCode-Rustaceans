// https://leetcode.com/problems/merge-intervals/solutions/1952859/rust-solution/
impl Solution {
 pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut iv = intervals;
  iv.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
  let (start, end) = (0, 1);
  let mut i = 0;
  loop {
   if i >= iv.len() - 1 {
    break;
   }
   let (cur, next) = (&iv[i], &iv[i + 1]);
   if cur[start] <= next[start] {
    if cur[end] >= next[start] {
     iv[i + 1] = vec![cur[start], std::cmp::max(cur[end], next[end])];
     iv.remove(i);
     continue;
    }
   }
   i += 1;
  }
  iv
 }
}
