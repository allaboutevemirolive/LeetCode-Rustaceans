// https://leetcode.com/problems/generate-parentheses/solutions/1978611/rust-recursion/
fn parentheses(mut string: String, left: i32, right: i32) -> Vec<String> {
  if right == 0 && left == 0 {
    return vec![string];
  }
  let mut result = Vec::new();
  if left > 0 {
    let mut new_string = string.clone();
    new_string.push('(');
    for r in parentheses(new_string, left-1, right).into_iter() {
      result.push(r);
    }
  }
  if left < right && right > 0 {
    string.push(')');
    for r in parentheses(string, left, right-1).into_iter() {
      result.push(r);
    }
  }
  result
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    parentheses("".into(), n, n)
}