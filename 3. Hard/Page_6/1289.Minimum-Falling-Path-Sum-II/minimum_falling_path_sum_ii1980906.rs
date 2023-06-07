// https://leetcode.com/problems/minimum-falling-path-sum-ii/solutions/1980906/rust-solution/
impl Solution {
  pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let m = grid[0].len();
    let inf = 1_000_000_000;
    let mut min = inf;
    let mut memo = vec![inf;m];
  
    for i in 0..m {
      memo[i] = std::cmp::min(memo[i], grid[0][i]);
    }
  
    for i in 1..n {
      let mut new_memo = vec![inf;m];
      for j in 0..m {
        let v = memo[j];
        for k in 0..m {
          if j == k { continue }
          new_memo[k] = std::cmp::min(new_memo[k], v + grid[i][k]);
        }
      }
      memo = new_memo;
    }
  
    for v in memo {
      min = std::cmp::min(min, v);
    }
  
    min
  }
}