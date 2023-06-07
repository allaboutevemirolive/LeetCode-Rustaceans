// https://leetcode.com/problems/stone-game-iii/solutions/3567716/rust-python-memoization/
impl Solution {

  fn play(arr: &Vec<i32>, cache: &mut Vec<Vec<i32>>, pos: usize, is_alice: usize) -> i32 {
    if pos >= arr.len() {
      return 0;
    }

    if cache[is_alice][pos] != i32::MAX {
      return cache[is_alice][pos];
    }

    let (mut res, mut sum) = (i32::MAX, 0);
    if is_alice == 1 {
      res = i32::MIN;
    }
    for i in pos .. (pos + 3).min(arr.len()) {
      sum += arr[i];
      if is_alice == 1 {
        res = res.max(Self::play(arr, cache, i + 1, 0) + sum);
      } else {
        res = res.min(Self::play(arr, cache, i + 1, 1) - sum);
      }
    }

    cache[is_alice][pos] = res;
    return res;
  }

  pub fn stone_game_iii(arr: Vec<i32>) -> String {
    let mut cache = vec![
      vec![i32::MAX; arr.len()],
      vec![i32::MAX; arr.len()],
    ];
    let res = Self::play(&arr, &mut cache, 0, 1);

    if res == 0 {
      return "Tie".to_string();
    } else if res > 0 {
      return "Alice".to_string();
    }
    return "Bob".to_string();
  }
}