// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/487647/rust-solution/
struct Helper {
	inorder: Vec<i32>,
	postorder: Vec<i32>,
	index: i32,
}

impl Helper {
	fn helper(&mut self, ini: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {
		if ini > end {
			return None;
		}

		let mut node = TreeNode::new(self.postorder[self.index as usize]);
		self.index -= 1;
		if ini == end {
			return Some(Rc::new(RefCell::new(node)));
		}

		let index = self.inorder.iter().position(|&e| e == node.val).unwrap() as i32;

		node.right = self.helper(index + 1, end);
		node.left = self.helper(ini, index - 1);
		Some(Rc::new(RefCell::new(node)))
	}
}

impl Solution {
	pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		let index = postorder.len() as i32 - 1;

		let mut h = Helper {
			inorder,
			postorder,
			index,
		};

		h.helper(0, index)
	}
}