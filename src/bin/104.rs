//Definition for a binary tree node.
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recu(node: &Option<Rc<RefCell<TreeNode>>>) -> i32{
            match node {
                None => 0,
                Some(node_wrapper) => {
                    let a_node = node_wrapper.borrow();
                    1 + recu(&a_node.left).max(recu(&a_node.right))
                }
            }
        }
    return recu(&root);
    }
}

pub fn main() {

    println!("main");
}