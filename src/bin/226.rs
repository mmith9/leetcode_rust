// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn invert_node(node: &Option<Rc<RefCell<TreeNode>>>) {
            if let Some(wrapper) = node {
                {
                    let mut nextnode = wrapper.borrow_mut();
                    (nextnode.left, nextnode.right) = (nextnode.right.take(), nextnode.left.take());
                }
                let nextnode = wrapper.borrow();
                invert_node(&nextnode.left);
                invert_node(&nextnode.right);
            }
        }
        invert_node(&root);
        return root;
    }
}


pub fn main() {}
