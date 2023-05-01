// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/2919986/rust-macro-rules/
impl Solution {
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let stack = &mut [0; 10_000];
    let mut n = 0;
    for tok in tokens {
        macro_rules! handle {
            { $($op:tt)+ } => (
                match tok.as_str() {
                    $(
                        stringify!($op) => {
                            n -= 2;
                            stack[n] = stack[n] $op stack[n+1];
                            // stack[n] $op= stack[n+1]; // can't concat 2 Rust tokens
                        }
                    )+
                    num => stack[n] = num.parse().unwrap(),
                }
            )
        }
        handle!{ + - * / }
        n += 1;
    }
    stack[n-1]
}
}