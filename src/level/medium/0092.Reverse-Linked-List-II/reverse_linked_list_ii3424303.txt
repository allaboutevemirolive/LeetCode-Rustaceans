// https://leetcode.com/problems/reverse-linked-list-ii/solutions/3424303/rust-o-n-runtime-o-n-space-decent-solution-0-1ms-no-copy/
type Node = Box<ListNode>;

impl Solution {
    /// Reverse the list while `right` > -1.
    pub fn reverse(head: &mut Option<Node>, mut n: i32) -> Option<Node> {
        let mut reversed = None;
        while let Some(node) = head.as_mut() {
            if n > -1 {
                let prev_reversed = reversed.take();
                let next = node.next.take();
                reversed = head.take();
                if let Some(reversed) = reversed.as_mut() {
                    reversed.next = prev_reversed;
                }
                *head = next;
            } else {
                break;
            }
            n -= 1;
        }
        reversed
    }

    /// Extract nodes from `head` until the `n`th node.
    fn extract_nodes(head: &mut Option<Node>, mut n: i32) -> Option<Node> {
        let mut new: Option<Node> = None;
        while n > 1 {
            if let Some(mut node) = head.take() {
                let next = Self::extract_nodes(&mut node.next, n - 1);
                *head = node.next;
                node.next = next;
                new.replace(node);
                return new;
            }
            n -= 1;
        }
        new
    }

    /// Append `to_append` to `head`.
    fn append_list(head: &mut Option<Node>, to_append: Option<Node>) -> Option<Node> {
        let mut new: Option<Node> = None;
        if let Some(mut node) = head.take() {
            let next = Self::append_list(&mut node.next, to_append);
            *head = node.next;
            node.next = next;
            new.replace(node);
            return new;
        } else {
            return to_append;
        }
    }

    pub fn reverse_between(mut head: Option<Node>, left: i32, right: i32) -> Option<Node> {
        let mut left_part = Self::extract_nodes(&mut head, left);
        let mut reversed = Self::reverse(&mut head, right - left);
        // After having extracted the untouched left part 
        // and reversed the nodes betwwen [left,right],
        // what is left of head is the right part of the final list.
        let right_part = head;
        let appended = Self::append_list(&mut reversed, right_part);
        let result = Self::append_list(&mut left_part, appended);
        result
    }
}
