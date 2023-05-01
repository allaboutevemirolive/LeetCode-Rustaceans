// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/1975553/rust/
fn eval_rpn(tokens: Vec<String>) -> i32 {
	let mut stack = Vec::with_capacity(tokens.len());

	for token in tokens {
		match token.as_str() {
			"+" | "-" | "*" | "/" => {
				let right: i32 = stack.pop().unwrap();
				let left: i32 = stack.pop().unwrap();
				stack.push(match token.as_str() {
					"+" => left + right,
					"-" => left - right,
					"*" => left * right,
					"/" => left / right,
					_ => unreachable!()
				});
			}
			_ => stack.push(token.parse().unwrap())
		}
	}
	stack.pop().unwrap()
}