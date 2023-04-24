// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/2531202/rust-simple-straitforward-0ms-solution/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeSet;

fn gen(bts: &mut BTreeSet<(i32, i32, i32, i32)>, n: &Option<Rc<RefCell<TreeNode>>>, r: i32, c: i32, uid: &mut i32) {
    if let Some(rrc) = n {
        let tn = rrc.borrow();
        bts.insert((c, r, tn.val, *uid));
        *uid += 1;
        gen(bts, &tn.left, r + 1, c - 1, uid);
        gen(bts, &tn.right, r + 1, c + 1, uid);
    }
}

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        // col, row, val, uid
        let mut bts: BTreeSet<(i32, i32, i32, i32)> = BTreeSet::new();
        
        // populate bts
        let mut uid: i32 = i32::MIN;
        gen(&mut bts, &root, 0, 0, &mut uid);
        
        // populate res 
        let mut res = Vec::new();
        let mut prevcol = i32::MIN;
        for (c, r, v, _) in bts {
            if c != prevcol {
                res.push(vec![]);
            }
            let ss = res.last_mut().unwrap();
            ss.push(v);
            prevcol = c;
        }
        
        res
    }
}