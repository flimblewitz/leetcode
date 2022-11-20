fn main() {
    let mut tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
    })));
    Solution::recover_tree(&mut tree);
    println!("{:?}", tree);
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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        // so I need to traverse the tree and pass along a list of traversed values
        // the one on the left will be the first one greater than the node to its right
        // the one on the right will be the last one less than the node to its left
        // 12345 _
        // 12354 v
        // 12543 v
        // 52341 v
        let v = get_ordered_nodes(root);
        // println!("{:?}", v);

        let left = v
            .iter()
            .enumerate()
            .find(|(index, node)| node.borrow().val > v[index + 1].borrow().val)
            .unwrap()
            .1;
        let right = v
            .iter()
            .enumerate()
            .rev()
            .find(|(index, node)| node.borrow().val < v[index - 1].borrow().val)
            .unwrap()
            .1;

        // println!("{:?}", left);
        // println!("{:?}", right);
        let mut left_borrow = left.borrow_mut();
        let tmp = left_borrow.val;
        // println!("{:?}", tmp_left_borrow);
        let mut right_borrow = right.borrow_mut();
        left_borrow.val = right_borrow.val;
        // println!("{:?}", left);
        // println!("{:?}", left_borrow);
        // println!("{:?}", tmp_left_borrow);
        // right_borrow.val = tmp_left_borrow.val;
        right_borrow.val = tmp;
        // println!("{:?}", right_borrow);
    }
}

fn get_ordered_nodes(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Rc<RefCell<TreeNode>>> {
    let mut v = vec![];
    if let Some(root) = root {
        v.append(&mut get_ordered_nodes(&root.borrow().left));
        v.push(root.clone());
        v.append(&mut get_ordered_nodes(&root.borrow().right));
    }
    v
}
