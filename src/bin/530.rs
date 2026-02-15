//Definition for a binary tree node.
struct Solution;


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


use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        let mut values:Vec<i32> = Vec::new();
        Self::walk_tree(&root, &mut values);
        values.sort();
        let mut res = i32::MAX;
        for pair in values.windows(2) {
            if let [a,b] = pair {
                res = res.min((a-b).abs());
            }
        }
        return res
    }

    fn walk_tree(maybe_node: &Option<Rc<RefCell<TreeNode>>>, values:&mut Vec<i32>) {
        match maybe_node {
            None => {return}
            Some(node) => {
                let borrowed = node.borrow();
                values.push(borrowed.val);
                Self::walk_tree(&borrowed.left, values);
                Self::walk_tree(&borrowed.right, values);
            }
        }
    }

    fn walk_tree2(maybe_node: &Option<Rc<RefCell<TreeNode>>>, values:&mut Vec<i32>) {
        if let Some(node) = maybe_node {
            let borrowed = node.borrow();
            values.push(borrowed.val);
            Self::walk_tree(&borrowed.left, values);
            Self::walk_tree(&borrowed.right, values);
            }
    }
}


pub fn main() {
    println!("boo");
}