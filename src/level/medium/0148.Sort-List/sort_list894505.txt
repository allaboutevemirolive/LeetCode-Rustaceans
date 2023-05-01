// https://leetcode.com/problems/sort-list/solutions/894505/rust-simple-solution/
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let current = &mut head;
        let mut heap = BinaryHeap::new();
        while let Some(node) = current {
            heap.push(Reverse(node.val));
            *current = node.next.take();
        }

        let mut head: Option<Box<ListNode>> = None;
        let mut current: &mut Option<Box<ListNode>> = &mut None;

        while !heap.is_empty() {
            let new_node = Some(Box::new(ListNode::new(heap.pop().unwrap().0)));
            if let Some(c) = current {
                c.next = new_node;
                current = &mut c.next;
            } else {
                head = new_node;
                current = &mut head;
            }
        }

        head
    }