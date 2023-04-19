// https://leetcode.com/problems/serialize-and-deserialize-binary-tree/solutions/969099/rust-bfs/
impl Codec {
    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res: Vec<String> = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);

        while let Some(entity) = queue.pop_front() {
            if let Some(ref node) = entity {
                res.push(node.borrow().val.to_string());
                queue.push_back(node.borrow().left.clone());
                queue.push_back(node.borrow().right.clone());
            } else {
                res.push(String::from("null"));
            }
        }
        res.join(",")
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let lst: Vec<Option<i32>> = data
            .split(",")
            .map(|s| {
                if s == "null" {
                    None
                } else {
                    Some(s.parse::<i32>().unwrap())
                }
            })
            .collect();

        let root = match lst[0] {
            Some(a) => Rc::new(RefCell::new(TreeNode::new(a))),
            None => return None,
        };
        let mut cur = 1;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());
        while let Some(node) = queue.pop_front() {
            if let Some(a) = lst[cur] {
                let l = Rc::new(RefCell::new(TreeNode::new(a)));
                node.borrow_mut().left = Some(l.clone());
                queue.push_back(l);
            }
            cur += 1;
            if let Some(a) = lst[cur] {
                let r = Rc::new(RefCell::new(TreeNode::new(a)));
                node.borrow_mut().right = Some(r.clone());
                queue.push_back(r);
            }
            cur += 1;
        }
        Some(root)
    }
}