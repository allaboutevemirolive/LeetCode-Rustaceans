// https://leetcode.com/problems/reducing-dishes/solutions/2047568/rust-solution/
impl Solution {
  pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
    let mut plus = vec![];
    let mut minus = vec![];

    let mut plus_total = 0;
    for v in satisfaction {
      if 0 <= v {
        plus.push(v);
        plus_total += v;
      } else {
        minus.push(v);
      }
    }

    plus.sort();
    minus.sort();
    minus.reverse();

    let mut result = 0;
    for i in 0..plus.len() {
      result += (i+1) as i32 * plus[i];
    }
    let base = result;

    let n = minus.len();
    for i in 0..n {
      let mut temp = 0;
      for j in 0..=i {
        let num = (i+1-j) as i32;
        let v = num * minus[j];
        temp += v;
      }
      
      let add = plus_total * (i+1) as i32;
      let v = add + temp;
      result = std::cmp::max(result, base + v);
    }

    result
  }
}