// https://leetcode.com/problems/brace-expansion-ii/solutions/487991/rust-12ms-2-2mb-translate-from-java/
use std::collections::HashSet;
// translate from https://leetcode.com/problems/brace-expansion-ii/discuss/348541/JAVA-iter_dfs-36ms
pub fn brace_expansion_ii(expression: String) -> Vec<String> {
    fn dfs(stack: &mut Vec<String>, set: &mut HashSet<String>, ans: &mut Vec<String>) {
        while !stack.is_empty() {
            let mut str = stack.pop().unwrap();
            if str.find('{') == None {
                if !set.contains(&str)
                {
                    set.insert(str.clone());
                    ans.push(str.clone());
                }
                continue;
            }
            let mut i = 0;
            let mut l = 0;
            let mut r = 0;
            while let Some(ch) = str[i..].chars().next() {
                if ch == '}' { break; }
                i += 1;
                if ch == '{' { l = i - 1; }
            }
            r = i;

            let args: Vec<&str> = str[l + 1..r].split(",").collect();
            for s in args {
                let mut ss = String::new();
                ss.push_str(&str[0..l]);
                ss.push_str(s);
                ss.push_str(&str[r + 1..]);
                stack.push(ss);
            }
        }
    }
    let mut ans = Vec::<String>::new();
    let mut stack = Vec::<String>::new();
    let mut set = HashSet::<String>::new();

    stack.push(expression);
    dfs(&mut stack, &mut set, &mut ans);
    ans.sort();

    ans
}
