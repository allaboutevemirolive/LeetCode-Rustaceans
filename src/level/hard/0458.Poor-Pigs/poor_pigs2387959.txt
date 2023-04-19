// https://leetcode.com/problems/poor-pigs/solutions/2387959/dynamic-programming-solution-rust/
use std::collections::HashMap;

impl Solution {
  pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
    if buckets == 1 {
      return 0
    }

    let count = minutes_to_test / minutes_to_die;
    let mut dp = HashMap::<(i32, i32), i32>::new();
    let mut combo = HashMap::<(i32, i32), i32>::new();

    let mut pigs = 1;
    loop {
      let max_buckets = Self::helper(&mut dp, &mut combo, pigs, count);
      if max_buckets >= buckets {
        return pigs
      }
      pigs += 1;
    }
  }

  fn helper(dp: &mut HashMap<(i32, i32), i32>, combo: &mut HashMap<(i32, i32), i32>, pigs: i32, count: i32) -> i32 {
    if pigs == 0 {
      return 1
    } 

    let key = (pigs, count);
    if dp.contains_key(&key) {
      return *dp.get(&key).unwrap()
    }

    let result = if count == 1 {
      2i32.pow(pigs as u32)
    } else {
      let mut sum = 0;
      for i in 0..=pigs {
        let x = Self::combo(combo, pigs, i) * Self::helper(dp, combo, i, count - 1);
        sum += x;
      }
      sum
    };
    dp.insert(key, result);
    result
  }

  fn combo(combo: &mut HashMap<(i32, i32), i32>, n: i32, k: i32) -> i32 {
    if k == 0 {
      return 1
    } else if k == 1 {
      return n
    } else if k == n {
      return 1
    }

    let key = (n, k);
    if combo.contains_key(&key) {
      *combo.get(&key).unwrap()
    } else {
      let result = Self::combo(combo, n - 1, k - 1) + Self::combo(combo, n - 1, k);
      combo.insert(key, result);
      result
    }
  }
}