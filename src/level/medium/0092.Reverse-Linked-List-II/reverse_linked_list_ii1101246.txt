// https://leetcode.com/problems/reverse-linked-list-ii/solutions/1101246/rust-safe-and-iterative-suggestions-needed-100-o-n/
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, from: i32, to: i32) -> Option<Box<ListNode>> {
    
    assert!(from <= to);

    // Skip to the head of the segment that has to be reversed
    let mut cursor = head;
    let mut pos = 1;
    
        
    // Duplicate 1 (see below):
    let mut A: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0))); // dummy node
    let mut A_tail = A.as_mut().unwrap();

    while let Some(mut x) = cursor {
        if pos == from {
            cursor = Some(x); // I do not want to do anything with it so move it back
            break;
        } else {
            pos += 1;
            cursor = x.next.take(); // cursor remains owner of the emerging B
            A_tail.next = Some(x);
            A_tail = A_tail.next.as_mut().unwrap(); // A_tail should now be looking at the newly added element
        }
    }
    
    // Duplicate 2 (see below):
    // `cursor` is now the head of B
    let mut B = Some(Box::new(ListNode::new(0)));
    let mut B_tail = B.as_mut().unwrap();

    while let Some(mut x) = cursor {
        if pos == to + 1 {
            cursor = Some(x);
            break;
        } else {
            pos += 1;
            cursor = x.next.take();
            B_tail.next = Some(x);
            B_tail = B_tail.next.as_mut().unwrap();
        }
    }

    let C = cursor; // When skipping finishes below, cursor will be the head of C

    cursor = Self::rev(B.unwrap().next);
        
    // Duplicate 3 (see below): Only without pos counter
    let mut rev_B = Some(Box::new(ListNode::new(0))); // rev(B) will be reaccumulated here
    let mut rev_B_tail = rev_B.as_mut().unwrap();

    while let Some(mut x) = cursor {
        cursor = x.next.take();
        rev_B_tail.next = Some(x);
        rev_B_tail = rev_B_tail.next.as_mut().unwrap();
    }

    rev_B_tail.next = C; // Connect B with C
    A_tail.next = rev_B.unwrap().next; // Connect A with B (skip B's dummy node)

    return A.unwrap().next; // Skip A's dummy node and return A -> rev(B) -> C
    }
    
    
    fn rev(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed: Option<Box<ListNode>> = None;
        let mut curr = head;

        while let Some(mut boxed_node) = curr {
            let mut tmp = boxed_node.next.take();
            curr = tmp;
            boxed_node.next = reversed.take();
            reversed = Some(boxed_node);
        }

        reversed
    }
}