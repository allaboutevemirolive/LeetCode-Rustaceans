// https://leetcode.com/problems/parsing-a-boolean-expression/solutions/647629/rust-clean-recursive-solution-runs-in-0ms-o-n-without-shared-state/
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let (rest, value) = parse_expression(&expression);

        if !rest.is_empty() {
            panic!("Invalid expression: {}", expression);
        }

        value
    }
}

fn parse_expression(expression: &str) -> (&str, bool) {
    match expression {
        expr if expr.starts_with('t') => (&expression[1..], true),
        expr if expr.starts_with('f') => (&expression[1..], false),
        expr if expr.starts_with('!') => {
            let (rest, inner) = get_inner_expression(&expression[1..]);
            if inner.len() != 1 {
                panic!("Invalid expression: {}", expression);
            }
            let value = !inner[0];
            (rest, value)
        }
        expr if expr.starts_with('&') => {
            let (rest, inner) = get_inner_expression(&expression[1..]);
            let value = inner.iter().fold(true, |acc, x| acc & x);
            (rest, value)
        }
        expr if expr.starts_with('|') => {
            let (rest, inner) = get_inner_expression(&expression[1..]);
            let value = inner.iter().fold(false, |acc, x| acc | x);
            (rest, value)
        }
        _ => panic!("Invalid expression: {}", expression),
    }
}

fn get_inner_expression(expression: &str) -> (&str, Vec<bool>) {
    if !expression.starts_with('(') {
        panic!("Invalid inner expression: {}", expression);
    }

    let mut expression = &expression[1..];
    let mut result: Vec<bool> = Vec::new();

    while !expression.starts_with(')') {
        let (rest, value) = parse_expression(expression);
        result.push(value);

        expression = if rest.starts_with(',') {
            &rest[1..]
        } else {
            rest
        };
    }

    (&expression[1..], result)
}