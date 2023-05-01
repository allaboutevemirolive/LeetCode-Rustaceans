// https://leetcode.com/problems/reverse-linked-list-ii/solutions/808421/rust-solution-no-stack-0ms/
impl Solution {
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        m: i32,
        n: i32,
    ) -> Option<Box<ListNode>> {
        if m == 1 {
            return Self::reverse(head, None, n - m + 1);
        }

        let mut count = 1;
        let mut current = head.as_mut();
        while let Some(node) = current {
            count += 1;

            if count == m {
                node.next = Self::reverse(node.next.take(), None, n - m + 1);
                break;
            } else {
                current = node.next.as_mut();
            }
        }

        head
    }

    fn reverse(
        head: Option<Box<ListNode>>,
        acc: Option<Box<ListNode>>,
        count: i32,
    ) -> Option<Box<ListNode>> {
        if count == 0 {
            return Self::append(acc, head);
        }

        if let Some(mut node) = head {
            let next = node.next;
            node.next = acc;

            Self::reverse(next, Some(node), count - 1)
        } else {
            acc
        }
    }

    fn append(
        mut front: Option<Box<ListNode>>,
        back: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut current = front.as_mut();
        while let Some(node) = current {
            if node.next.is_none() {
                node.next = back;
                break;
            }
            current = node.next.as_mut();
        }

        front
    }
}