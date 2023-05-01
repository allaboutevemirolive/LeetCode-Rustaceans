// https://leetcode.com/problems/generate-parentheses/solutions/2362081/rust/
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        Self::generate_parenthesis_helper(n, n, &mut String::new(), &mut Vec::new(), &mut res);
        res
    }
    
    pub fn generate_parenthesis_helper(left: i32, right: i32, mut built: &mut String, mut stack: &mut Vec<String>, mut res: &mut Vec<String>) {
        if right < left {
            return;
        }
        if left == 0 && right == 0 && stack.is_empty() {
            res.push(built.to_string());
        } else {
            if right > 0 {
                if let Some(c) = stack.last() {
                    let last_is_left = c == "(";
                    if last_is_left {
                        stack.pop();
                    }
                    built.push_str(")");
                    Self::generate_parenthesis_helper(left, right-1, built, stack, res);
                    if last_is_left {
                        stack.push("(".to_string());
                    }
                    built.pop();
                }
            }
            if left > 0 {
                stack.push("(".to_string());
                built.push_str("(");
                Self::generate_parenthesis_helper(left-1, right, built, stack, res);
                stack.pop();
                built.pop();
            }
        }
    }
}