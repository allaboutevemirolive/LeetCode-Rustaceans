// https://leetcode.com/problems/minimum-size-subarray-sum/solutions/3243515/rust-solution-using-binary-search/
type Target = i32;
type UseValue = i32;
fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
  let mut low = 0;
  let mut high = arr.len();
  while low != high {
    let mid = (low + high) / 2;
    match arr[mid].cmp(x) {
      std::cmp::Ordering::Less => {
        low = mid + 1;
      }
      std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
        high = mid;
      }
    }
  }
  low
}

impl Solution {
  pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut memo = vec![0;n+1];
    for i in 0..n {
      memo[i+1] = memo[i] + nums[i];
    }

    let default = 1000000;
    let mut result = default;
    let mut tot = 0;
    for i in 0..n {
      let tv = tot+target;
      let ni = lower_bound(&memo, &tv);
      if ni == n+1 {
        if tv <= memo[ni-1] {
          result = result.min(ni-i);
        }
        break
      }
      
      let ni = if tv <= memo[ni-1] {
        ni - 1
      } else {
        ni
      };
      
      result = result.min(ni-i);
      tot += nums[i];
    }

    if result == default {
      0
    } else {
      result as i32
    }
  }
}