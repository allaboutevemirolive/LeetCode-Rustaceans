// https://leetcode.com/problems/parsing-a-boolean-expression/solutions/2281831/rust-two-stacks/
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        if expression.len() == 1 {
            match expression.as_str() {
                "t" => return true,
                _ => return false,
            }
        }

        let expression: Vec<char> = expression.chars().filter(|c| *c != ',').collect();
        let (mut ops_stack, mut exp_stack) = (vec![expression[0]], vec![]);
        let mut i = 1;
        while ops_stack.len() > 0 {
            while expression[i] == 't' || expression[i] == 'f' || expression[i] == '(' {
                exp_stack.push(expression[i].clone());
                i += 1;
            }

            if expression[i] == ')' {
                let mut res_bool = match exp_stack[exp_stack.len()-1] {
                    't' => true,
                    _ => false
                };
                match ops_stack.pop().unwrap() {
                    '&' => {
                        loop {
                            match exp_stack.pop().unwrap() {
                                't' => res_bool &= true,
                                'f' => res_bool &= false,
                                _ => break,
                            }
                        }
                    }
                    '|' => {
                        loop {
                            match exp_stack.pop().unwrap() {
                                't' => res_bool |= true,
                                'f' => res_bool |= false,
                                _ => break,
                            }
                        }
                    }
                    _ => {
                        loop {
                            match exp_stack.pop().unwrap() {
                                't' => res_bool = false,
                                'f' => res_bool = true,
                                _ => break,
                            };
                        }
                    }
                }
                match res_bool {
                    true => exp_stack.push('t'),
                    false => exp_stack.push('f')
                }
            } else {
                ops_stack.push(expression[i]);
            }

            i += 1;
        }

        match exp_stack.pop().unwrap() {
            't' => true,
            _ => false,
        }
     }
}