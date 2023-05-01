// https://leetcode.com/problems/sort-list/solutions/477190/rust-solution-100-faster-4ms-100-less-mb-4-1-mb-elegant-solution/
pub fn recurse(head: Option<Box<ListNode>>, appending_node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => appending_node,
        Some (node) => {
            let value = node.val;
            let mut less_than_node = None;
            let mut more_than_node = None;
            let mut equal_node = None;
            let mut iter_node = Some (node);
            while let Some (mut node) = iter_node {
                let next_node = node.next;
                if node.val == value {
                    node.next = equal_node;
                    equal_node = Some (node);
                } else if node.val < value {
                    node.next = less_than_node;
                    less_than_node = Some (node);
                } else {
                    node.next = more_than_node;
                    more_than_node = Some (node);
                }
                iter_node = next_node
            };

            let sorted_more_than_node = recurse(more_than_node, appending_node);
            let mut last_iter_mut = equal_node.as_mut();
            while let Some (last_iter) = last_iter_mut {
                if let None = last_iter.next {
                    last_iter.next = sorted_more_than_node;
                    break
                }
                last_iter_mut = last_iter.next.as_mut();
            }

            recurse(less_than_node, equal_node)
        }
    }
}

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        recurse(head, None)
    }
}