// https://leetcode.com/problems/partition-list/solutions/1158517/partition-list-rust/
impl ListNode {
    fn push(&mut self, val: i32) {
        let node = ListNode {
            next: self.next.take(),
            val,
        };
        self.next = Some(Box::new(node));
    }

    fn from_vec(vec: Vec<i32>) -> Self {
        let mut node = ListNode::new(vec[0]);
        vec.iter().skip(1).rev().for_each(|x| node.push(*x));
        node
    }
}

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut tail_vec = Vec::new();
        let mut head_vec = Vec::new();

        let mut heads = &head;

        loop {
            match heads {
                Some(v) => {
                    if v.val >= x {
                        tail_vec.push(v.val)
                    } else {
                        head_vec.push(v.val)
                    }
                    heads = &v.next;
                }
                None => { break; }
            }
        }

        head_vec.append(&mut tail_vec);

        if head_vec.is_empty() {
            return head;
        }

        Some(Box::new(ListNode::from_vec(head_vec)))
    }
}
