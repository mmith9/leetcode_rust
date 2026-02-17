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
        return Solution::recu(0, &root);
    }

    fn recu(depth:i32, node: &Option<Rc<RefCell<TreeNode>>>) -> i32{
        let mut res:i32;
        match node {
            None => {return depth}
            Some(node_wrapper) => {
                let a_node = node_wrapper.borrow();
                res = Solution::recu(depth+1, &a_node.left);
                res = res.max(Solution::recu(depth+1, &a_node.right));
            }
        }
        return res;
    }
}

pub fn main() {

    println!("main");
}