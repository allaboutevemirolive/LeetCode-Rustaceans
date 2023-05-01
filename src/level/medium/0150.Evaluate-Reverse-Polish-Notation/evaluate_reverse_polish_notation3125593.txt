// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/3125593/rust-stack-solution/
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let operators = String::from("+-*/");

        tokens.into_iter()
            .for_each(|token| {
                if !operators.contains(&token) {
                    stack.push(token.parse::<i32>().unwrap());
                } else {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();

                    match token.as_str() {
                        "+" => stack.push(b + a),
                        "-" => stack.push(b - a),
                        "*" => stack.push(b * a),
                        "/" => stack.push(b / a),
                        _ => {}
                    }
                }
            });

        *stack.last().unwrap()
    }
}