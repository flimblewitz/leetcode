fn main() {
    println!(
        "{:?}",
        Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))))
    );
    println!(
        "{:?}",
        Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))))
    );
    println!(
        "{:?}",
        Solution::is_valid_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }))))
    );
}

struct Solution {}
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
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        recursively_confirm_valid_bst(root).is_ok()
    }
}

fn recursively_confirm_valid_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Result<(bool, i32, i32), ()> {
    match root {
        None => Ok((true, 0, 0)),
        Some(inner) => {
            let root = inner.as_ref().borrow();
            let left_stats = recursively_confirm_valid_bst(root.left.clone())?;
            let right_stats = recursively_confirm_valid_bst(root.right.clone())?;
            match (left_stats, right_stats) {
                ((true, ..), (true, ..)) => Ok((false, root.val, root.val)),
                ((true, ..), (false, r_min, r_max)) if r_min > root.val => {
                    Ok((false, root.val, r_max))
                }
                ((false, l_min, l_max), (true, ..)) if l_max < root.val => {
                    Ok((false, l_min, root.val))
                }
                ((false, l_min, l_max), (false, r_min, r_max))
                    if l_max < root.val && r_min > root.val =>
                {
                    Ok((false, l_min, r_max))
                }
                _ => Err(()),
            }
        }
    }
}
