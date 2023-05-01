// https://leetcode.com/problems/simplify-path/solutions/3410423/rust-solution/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack:Vec<&str> = vec![];
        let mut input:Vec<&str>=path.split("/").collect();
        for curr in input {
//continue on dots and empty strings
            if curr=="." || curr==""{
                continue;
            }
//if curr is two dots we go up the directory by removing from the stack
            if curr==".."{
                stack.pop();
                continue;
            }
//push current directory
            stack.push(curr);
        }
        let mut res = "".to_string();
        while !stack.is_empty() {
            res.push('/');
            res.push_str(stack.remove(0));
        }
        if res.len()==0 {
            return "/".to_string();
        }
        res
    }
}