// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/solutions/2151483/rust-index-based-approach/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};
use std::mem::swap;

impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = VecDeque::with_capacity(1);
        // node, index
        queue.push_back((root.clone(), 0));
        let mut nodes = HashMap::new();
        let mut p_index = None;
        let mut q_index = None;

        while let Some((rc_node, i)) = queue.pop_front() {
            if rc_node.clone() == p {
                p_index = Some(i);
            } else if rc_node.clone() == q {
                q_index = Some(i);
            }

            if p_index.is_some() && q_index.is_some() {
                break;
            }

            if let Some(node) = rc_node.clone() {
                nodes.insert(i, rc_node);
                let left_i = i*2+1;
                queue.push_back((node.borrow().left.clone(), left_i));
                queue.push_back((node.borrow().right.clone(), left_i+1));
            }
        }
        let (mut p_index, mut q_index) = match (p_index, q_index) {
            (Some(p_i), Some(q_i)) => (p_i, q_i),
            _ => return None,
        };
        while p_index >= 0 && q_index >= 0 {
            if p_index == q_index {
                return nodes.get(&p_index).unwrap().clone();
            }
            if p_index < q_index {
                swap(&mut p_index, &mut q_index);
            }
			// binary tree level nodes correspond to the layer
            let layer = (p_index as f32 + 1.0).log2().floor() as u32;
			// node index within the layer
            let current_layer_index = p_index + 1 - 2_usize.pow(layer);
			// index of the node parent
            p_index = 2_usize.pow(layer.saturating_sub(1)) + current_layer_index / 2 - 1;
        }
        None
    }
}