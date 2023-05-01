// https://leetcode.com/problems/simplify-path/solutions/3408337/rust-solution/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let keys = path.split("/").filter(|s| s != &"").collect::<Vec<&str>>();
        let mut vec = vec![];
        let mut output_string = String::new();

        for key in keys {
            if key == "." {
                continue;
            } else if key == ".." {
                vec.pop();
            } else {
                vec.push(key);
            }
        }

        let len = vec.len();

        if len == 0 {
            output_string.push_str("/");
        } else {
            for key in vec {
                output_string.push_str(format!("/{}", key).as_str());
            }
        }

        return output_string;
    }
}
