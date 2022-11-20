fn main() {
    println!(
        "{:?}",
        Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: None,
            }))),
        }))),)
    );
    println!("{:?}", Solution::inorder_traversal(None));
    println!(
        "{:?}",
        Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode::new(1)))))
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = vec![];

        if let Some(root) = root {
            let root = root.borrow();
            v.append(&mut Self::inorder_traversal(root.left.clone()));
            v.push(root.val);
            v.append(&mut Self::inorder_traversal(root.right.clone()));
        }
        v
    }
}
