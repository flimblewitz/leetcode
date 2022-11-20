fn main() {
    // println!("{:?}", Solution::generate_trees(2));
    println!("{:?}", Solution::generate_trees(3));
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
use std::vec;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        recursively_generate_trees(1, n)
    }
}

fn recursively_generate_trees(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut v = vec![];
    for i in start..=end {
        let mut trees: Vec<TreeNode> = vec![];
        if i > start {
            for left in recursively_generate_trees(start, i - 1).into_iter() {
                trees.push(TreeNode {
                    val: i,
                    left,
                    right: None,
                })
            }
        } else {
            trees.push(TreeNode::new(i));
        }
        if i < end {
            let mut trees_with_right = vec![];
            for tree in trees.iter() {
                for right in recursively_generate_trees(i + 1, end).iter() {
                    trees_with_right.push(TreeNode {
                        val: tree.val,
                        left: tree.left.clone(),
                        right: right.clone(),
                    });
                }
            }
            trees = trees_with_right;
        }
        v.append(&mut trees);
    }
    v.into_iter()
        .map(|tn| Some(Rc::new(RefCell::new(tn))))
        .collect()
}
