// https://leetcode.com/problems/simplify-path/solutions/3406764/rust-solution/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();

        for dir in path.split('/') {
            match dir {
                "" => {},
                "." => {},
                ".." => { stack.pop(); stack.pop(); } 
                v => { stack.push("/"); stack.push(dir); },
            }
        }

        if stack.is_empty() { "/".to_owned() } else { stack.join("") }
    }
}