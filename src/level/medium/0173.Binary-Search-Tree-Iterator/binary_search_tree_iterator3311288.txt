// https://leetcode.com/problems/binary-search-tree-iterator/solutions/3311288/rust-dfs-inorder-solution/
use std::cell::RefCell;
use std::rc::Rc;
struct BSTIterator {
  nodes:Vec<i32>,
  index:i32
}

impl BSTIterator {

  fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
    let mut nodes : Vec<i32> =vec![];
    BSTIterator::dfs(&mut nodes,root);
    Self{nodes,index:-1}
  }

  fn dfs(nodes: &mut Vec<i32>, root:Option<Rc<RefCell<TreeNode>>>){
    if root.is_none(){
      return;
    }
    BSTIterator::dfs(nodes,root.clone().unwrap().borrow().left.clone());
    nodes.push(root.clone().unwrap().borrow().val);
    BSTIterator::dfs(nodes,root.unwrap().borrow().right.clone());
  }

  fn next(&mut self) -> i32 {
    self.index+=1;
    self.nodes[self.index as usize]
  }

  fn has_next(&self) -> bool {
    self.index<self.nodes.len() as i32 -1
  }
}