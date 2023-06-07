// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/solutions/3451913/rust-fun-recursive-format/
use std::rc::Rc;
use std::cell::RefCell;

impl Default for TreeNode {
    fn default() -> Self {
        Self::new(0)
    }
}

struct Codec;
impl Codec {
    fn new() -> Self {
        Self
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        root.map(|r| {
            let r = r.borrow();
            format!("({}<{}>{})",
                self.serialize(r.left.clone()),
                r.val,
                self.serialize(r.right.clone()),
            )
        }).unwrap_or_default()
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut val = String::new();
        let mut curr = None;
        for c in data.chars() {
            match c {
                '(' => {
                    stack.push(curr.unwrap_or_default());
                    curr = None;
                }
                ')' => {
                    let parent = stack.pop()?;
                    parent.borrow_mut().right = curr;
                    curr = Some(parent);
                }
                '<' => {
                    let parent = stack.pop()?;
                    val = String::new();
                    parent.borrow_mut().left = curr;
                    curr = Some(parent);
                }
                '>' => {
                    let parent = curr?;
                    parent.borrow_mut().val = val.parse().ok()?;
                    stack.push(parent);
                    curr = None;
                }
                x => val.push(x),
            }
        }
        return curr;
    }
}