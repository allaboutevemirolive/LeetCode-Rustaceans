// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/2921299/rust-solution/
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut exec_stack : Vec<i32> = vec![];

        for token in tokens.iter() {
            // Trying to parse string to i32
            if let Ok(val) = token.parse() {
                exec_stack.push(val);
            } else {
                let operand_2 = exec_stack.pop().unwrap();
                let operand_1 = exec_stack.pop().unwrap();

                match token.as_str() {
                    "+" => exec_stack.push(operand_1+operand_2),
                    "-" => exec_stack.push(operand_1-operand_2),
                    "*" => exec_stack.push(operand_1*operand_2),
                    "/" => exec_stack.push(operand_1/operand_2),
                    _ => {}
                }
            }
        }
        exec_stack[0]
    }
}