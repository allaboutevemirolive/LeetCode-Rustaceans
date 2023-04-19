// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/solutions/965824/rust-dfs-iteratively-recursively/
pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
	let mut res = vec![];
	let mut stack = vec![root];
	while let Some(node) = stack.pop() {
		let node = match node {
			Some(a) => a,
			None => {
				res.push("null".to_string());
				continue;
			}
		};
		res.push(node.borrow().val.to_string());
		stack.push(node.borrow().right.clone());
		stack.push(node.borrow().left.clone());
	}
	res.join(",")
}