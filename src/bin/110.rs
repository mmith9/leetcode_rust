// // Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }

// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
// use std::rc::Rc;
// use std::cell::RefCell;
// struct Solution;

// impl Solution {
  
//     pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
//         let mut mindepth :i32 = i32::MAX;
//         let mut maxdepth :i32 = 0;




//         }


        
//         return true;
//     }

//     fn recu(depth:i32, node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
//       match node {
//         None => {return false}
//         Some(a_node) => {
//             let r = recu(depth+1, a_node.right);
//             let l = recu(depth+1, a_node.left);
//             return true;
//         }
//     }
//   }


pub fn main() {

}

