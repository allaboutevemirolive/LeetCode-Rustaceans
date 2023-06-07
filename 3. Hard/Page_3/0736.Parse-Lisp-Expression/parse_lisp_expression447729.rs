// https://leetcode.com/problems/parse-lisp-expression/solutions/447729/rust-recursive-0-ms-2-6-mb/
use std::collections::HashMap;

impl Solution {
    fn expr<'a>(expr_stream: &mut Vec<&'a str>, mut variables: HashMap<&'a str, i32>) -> i32 {
        while let Some(action) = expr_stream.pop() {
            // Closing brackets are only important for let operator and it's handled there.
            // For main operator loop, it can be ignored.
            if action == ")" {
                continue;
            }

            // Start of new scope
            if action == "(" {
                return Solution::expr(expr_stream, variables.clone());
            }

            // (add 23 -20) = 3
            if action == "add" {
                let op1 = Solution::expr(expr_stream, variables.clone());
                let op2 = Solution::expr(expr_stream, variables.clone());

                // Remove ")"
                expr_stream.pop();
                
                return op1 + op2;
            }
            
            // (mult 3 4) = 12
            if action == "mult" {
                let op1 = Solution::expr(expr_stream, variables.clone());
                let op2 = Solution::expr(expr_stream, variables.clone());
                
                // Remove ")"
                expr_stream.pop();

                return op1 * op2;
            }

            // (let x 2 x -3 x 4 (mult x 5)) = 20
            if action == "let" {
                while let Some(var_name) = expr_stream.pop() {
                    // we are facing evaluation, if variable name is "("
                    if var_name == "(" {
                        return Solution::expr(expr_stream, variables.clone());
                    }

                    // get value what should be assigned to variable
                    match expr_stream.pop() {
                        Some(")") => {
                            expr_stream.push(var_name);

                            return Solution::expr(expr_stream, variables.clone());
                        },
                        Some("(") => {
                            variables.insert(var_name, Solution::expr(expr_stream, variables.clone()));
                        },
                        Some(var_value) => {
                            expr_stream.push(var_value);
                            variables.insert(var_name, Solution::expr(expr_stream, variables.clone()));                            
                        },
                        _ => {
                            panic!("Let operator format is not valid!");
                        }
                    };
                }
            }

            // Handle integers
            if let Ok(value) = String::from(action).parse::<i32>() {
                return value;
            }

            // Handle variables
            return variables.get(action).unwrap_or(&0i32).clone();
        }

        panic!("Invalid list code!");
    }

    pub fn evaluate(expression: String) -> i32 {
        let mut expression = expression;

        // Add spaces around brackets for better split by space
        expression = expression.replace("(", " ( ");
        expression = expression.replace(")", " ) ");

        // Reverse order is used because I have storing data in vector.
        // Reason for using of vector is easier handling of let operator.
        // And I am using pop for fetching next operator.
        // It's also possible to use Iterator instead of Vec, but handling of let is a bit trickier.
        Solution::expr(&mut expression.split_whitespace().rev().collect::<Vec<&str>>(), HashMap::new())
    }
}