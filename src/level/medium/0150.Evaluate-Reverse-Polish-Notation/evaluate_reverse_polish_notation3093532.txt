// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/3093532/rust-solution/
impl Solution {
  pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let plus = format!("+");
    let minus = format!("-");
    let multiple = format!("*");
    let divide = format!("/");
    let mut stack = vec![];
    for v in tokens {
      if v == plus {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b+a);
      } else if v == minus {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b-a);
      } else if v == multiple {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b*a);
      } else if v == divide {
        let a = stack.pop().unwrap();
        let b = stack.pop().unwrap();
        stack.push(b/a);
      } else {
        stack.push(v.parse::<i32>().unwrap());
      }
    }
    stack[0]
  }
}