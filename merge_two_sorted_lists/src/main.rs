fn main() {
    // list1 = [1,2,4], list2 = [1,3,4]
    let list1 = new_opt_box_list_node(1, new_opt_box_list_node(2, new_opt_box_list_node(4, None)));
    let list2 = new_opt_box_list_node(1, new_opt_box_list_node(3, new_opt_box_list_node(4, None)));
    println!("{}", Solution::merge_two_lists(list1, list2).unwrap());
}

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, ", self.val).and(match &self.next {
            Some(box_node) => box_node.fmt(f),
            None => Ok(()),
        })
    }
}

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(box_node2)) => Some(box_node2),
            (Some(box_node1), None) => Some(box_node1),
            (Some(box_node1), Some(box_node2)) => {
                if box_node1.val < box_node2.val {
                    new_opt_box_list_node(
                        box_node1.val,
                        Self::merge_two_lists(box_node1.next, Some(box_node2)),
                    )
                } else {
                    new_opt_box_list_node(
                        box_node2.val,
                        Self::merge_two_lists(Some(box_node1), box_node2.next),
                    )
                }
            }
        }
    }
}

fn new_opt_box_list_node(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode { val, next }))
}
