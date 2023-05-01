// https://leetcode.com/problems/simplify-path/solutions/3406879/rust-stack-solution/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = vec![];

        path.split("/").for_each(|s| match s {
            "." | "" => (),
            ".." => {
                stack.pop();
            },
            _ => stack.push(s),
        });

        "/".to_string() + &stack.join("/")
    }
}