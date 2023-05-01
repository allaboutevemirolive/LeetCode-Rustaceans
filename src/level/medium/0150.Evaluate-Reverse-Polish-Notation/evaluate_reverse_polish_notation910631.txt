// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/910631/rust-one-iter/
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        tokens
            .iter()
            .fold(Vec::new(), |mut acc, token| {
                match token.as_str() {
                    "+" => {
                        let right = acc.pop().unwrap();
                        let left = acc.pop().unwrap();
                        acc.push(left + right);
                    }
                    "-" => {
                        let right = acc.pop().unwrap();
                        let left = acc.pop().unwrap();
                        acc.push(left - right);
                    }
                    "*" => {
                        let right = acc.pop().unwrap();
                        let left = acc.pop().unwrap();
                        acc.push(left * right);
                    }
                    "/" => {
                        let right = acc.pop().unwrap();
                        let left = acc.pop().unwrap();
                        acc.push(left / right);
                    }
                    _ => acc.push(token.parse().unwrap()),
                };
                acc
            })
            .pop()
            .unwrap()
    }
}