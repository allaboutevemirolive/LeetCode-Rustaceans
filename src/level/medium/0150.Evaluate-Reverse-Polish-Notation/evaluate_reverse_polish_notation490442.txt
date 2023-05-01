// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/490442/rust-simple-solution/
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for token in tokens.iter() {
            if let Ok(n) = token.parse() {
                stack.push(n);
            } else {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                match token.as_str() {
                    "+" => stack.push(lhs + rhs),
                    "-" => stack.push(lhs - rhs),
                    "*" => stack.push(lhs * rhs),
                    "/" => stack.push(lhs / rhs),
                    _ => {}
                }
            }
        }
        stack[0]
    }
}