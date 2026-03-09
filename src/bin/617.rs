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
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(is1), Some(is2)) => {{
                let mut n1 = is1.borrow_mut();
                let n2 = is2.borrow();
                n1.left = Self::merge_trees(n1.left.clone(), n2.left.clone());
                n1.right = Self::merge_trees(n1.right.clone(), n2.right.clone());
                n1.val += n2.val;}
                return Some(is1);
            }
            (Some(is), None) | (None, Some(is)) => {
                return Some(is)
            }
            _ => {return None}
        }
    }
}

pub fn main() {

}