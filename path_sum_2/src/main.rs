fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        }))),
    })));
    println!("{:?}", Solution::path_sum(tree, 22));
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    println!("{:?}", Solution::path_sum(tree, 5));
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut v = vec![];
        if let Some(root) = root {
            let root = root.borrow();
            if root.val == target_sum && root.left.is_none() && root.right.is_none() {
                return vec![vec![root.val]];
            }
            let new_target_sum = target_sum - root.val;
            let mut left_paths = Self::path_sum(root.left.clone(), new_target_sum);
            let mut right_paths = Self::path_sum(root.right.clone(), new_target_sum);
            v.append(&mut left_paths);
            v.append(&mut right_paths);
            v.iter_mut().for_each(|path| path.insert(0, root.val));
        }
        v
    }
}
