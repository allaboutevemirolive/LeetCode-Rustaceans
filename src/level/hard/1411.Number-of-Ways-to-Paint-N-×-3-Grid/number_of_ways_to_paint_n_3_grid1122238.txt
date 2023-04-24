// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/solutions/1122238/rust-beats-100-100-no-explanation/
impl Solution {  
  pub fn num_of_ways(n: i32) -> i32 {
    let mut a = 0usize;
    let mut ab = 3usize;
    for _ in 0..n {
      let t = ab * 2;
      a += t;
      ab = (a + t) % 1_000_000_007;
    }
    ab as i32
  }
}