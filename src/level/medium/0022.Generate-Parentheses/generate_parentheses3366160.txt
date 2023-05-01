// https://leetcode.com/problems/generate-parentheses/solutions/3366160/rust-recursion/
impl Solution {
	fn foo(vec: &mut Vec<String>, left: i32, right: i32, cur: String) {
		if left < 0 || right < 0 {
			return;
		}
		if left < 1 && right < 1 {
			vec.push(cur.clone());
			return;
		}
    
    if left < right {
        Self::foo(vec, left-1, right, cur.clone()+"(");
        Self::foo(vec, left, right-1, cur.clone()+")");
    } else{ 
        Self::foo(vec, left-1, right, cur.clone()+"(");
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res : Vec<String> = Vec::new();
    Self::foo(&mut res, n, n, String::new());        
    return res;
}
}