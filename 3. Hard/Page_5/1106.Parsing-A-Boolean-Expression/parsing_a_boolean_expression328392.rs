// https://leetcode.com/problems/parsing-a-boolean-expression/solutions/328392/rust-0ms-faster-than-100/
/// We can solve this problem by implementing our own stack and treat (&, |, !) as a function calls
/// and (t, f) as params to these functions.
///
/// For example here is a simple Expression representing `not` function
/// input: "!(f)"
/// the parser will create an empty stack at first for this function
/// ```t
/// +---------+
/// |         |
/// +---------+
/// ```
/// after that the parser will match every `char` in the expression and try to push that `Expr`
/// in the stack
///
/// We have two type of `Expr` represented by the `Expr` enum.
/// so the parser will at first hit the `!` char, it will push `Expr::Func(not)` to the stack
///
/// ```t
/// +-----------------+
/// | Expr::Func(not) |
/// +-----------------+
/// ```
/// next it would match the `(` char, we ignore that, then matching the `f` char, it will map
/// that to `Expr::Const(false)` and push it to the stack
/// ```t
/// +--------------------+
/// |   Expr::Func(not)  |
/// +--------------------+
/// | Expr::Const(false) |
/// +--------------------+
/// ```
/// > Note that since our stack is implemented using `Vec` it pushes in the reverse order, but
/// the stack is pushes into the top.
///
/// next we he hit the `)` char, that's where we execute the expr.
///
/// we will start from the back (last element) in our `Vec` to act as a `Stack`, we start to
/// collect a new `Vec` of `bool`s called `params` from the stack and pop that value too from the
/// stack and once we hit a `Expr::Func` we stop.
/// `params = [false]`
/// ```t
/// +-----------------+
/// | Expr::Func(not) |
/// +-----------------+
/// ```
/// next we get that function from the stack, and call it with the collected `params`
/// ```t
/// +---------+
/// |         |
/// +---------+
/// ```
/// `result = Expr::Const(func(params))`
///
/// and we push the result pack in the stack
/// ```t
/// +-------------------+
/// | Expr::Const(true) |
/// +-------------------+
/// ```
/// and we go to next `char` and so on..
///
/// at the end of the loop we know that we will be left with one `Expr::Const` on the top of our
/// stack so we fetch it and return the result :)
///
pub fn parse_bool_expr(expression: String) -> bool {
    let mut stack: Vec<Expr> = Vec::new();
    for c in expression.chars() {
        match c {
            '!' => stack.push(Expr::Func(not)),
            '|' => stack.push(Expr::Func(or)),
            '&' => stack.push(Expr::Func(and)),
            't' => stack.push(Expr::Const(true)),
            'f' => stack.push(Expr::Const(false)),
            ')' => {
                let mut params: Vec<bool> = vec![];
                while let Some(Expr::Const(v)) = stack.last() {
                    params.push(*v);
                    stack.pop(); // then remove it
                }
                // get the last func for these params
                if let Expr::Func(func) = stack.pop().unwrap() {
                    let result = func(params);
                    // store the value back to the stack.
                    stack.push(Expr::Const(result));
                }
            }
            _ => {
                // I would ignore that
            }
        }
    }
    if let Expr::Const(v) = *stack.last().unwrap() {
        v
    } else {
        unreachable!("Oh Bad")
    }
}

fn not(tf: Vec<bool>) -> bool {
    !tf[0]
}

fn or(tf: Vec<bool>) -> bool {
    tf.iter().any(|v| *v)
}

fn and(tf: Vec<bool>) -> bool {
    tf.iter().all(|v| *v)
}

enum Expr {
    Const(bool),
    Func(fn(tf: Vec<bool>) -> bool),
}