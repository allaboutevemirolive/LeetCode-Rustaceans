// https://leetcode.com/problems/flatten-binary-tree-to-linked-list/solutions/1066173/rust-solution/
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut list = Vec::new();

        fn post_order(root: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<Rc<RefCell<TreeNode>>>) {
            if let Some(r) = root {
                list.push(r.clone());

                post_order(r.borrow_mut().left.take(), list);
                post_order(r.borrow_mut().right.take(), list);
            }
        }

        post_order(root.clone(), &mut list);

        for i in 1..list.len() {
            let mut pre = list[i - 1].clone();
            let mut cur = list[i].clone();

            pre.borrow_mut().left.take();

            pre.borrow_mut().right = Some(cur);
        }
    }