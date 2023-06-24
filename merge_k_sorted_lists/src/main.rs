fn main() {
    // lists = [[1,4,5],[1,3,4],[2,6]]
    let list1 = new_opt_box_list_node(1, new_opt_box_list_node(4, new_opt_box_list_node(5, None)));
    let list2 = new_opt_box_list_node(1, new_opt_box_list_node(3, new_opt_box_list_node(4, None)));
    let list3 = new_opt_box_list_node(2, new_opt_box_list_node(6, None));
    println!(
        "{}",
        Solution::merge_k_lists(vec![list1, list2, list3]).unwrap()
    );
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        // option 1
        // this is not especially efficient; it only beats ~12% of other solutions for runtime and ~6% for memory
        // let mut lists = lists;
        // _merge_k_lists_recursive(&mut lists)

        // option 2
        // maybe it would be better to not perform the mass comparison between every list at each step and instead I could just merge them one pair at a time, using the same code I wrote for the merge_two_sorted_lists problem
        // nah, it's not really any better. It only beats ~6% of other solutions for both runtime and memory
        // it still works, though, and it's a lot easier to read
        lists
            .into_iter()
            .fold(None, |acc, list| merge_two_lists(acc, list))
    }
}

fn new_opt_box_list_node(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode { val, next }))
}

fn merge_two_lists(
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
                    merge_two_lists(box_node1.next, Some(box_node2)),
                )
            } else {
                new_opt_box_list_node(
                    box_node2.val,
                    merge_two_lists(Some(box_node1), box_node2.next),
                )
            }
        }
    }
}

fn _merge_k_lists_recursive(lists: &mut Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    // so I have to look at each of these and compare them all by the root and val
    let (index_of_list_with_min_val, list_with_min_val) = lists
        .iter()
        .enumerate()
        .filter_map(|(index, list)| list.as_ref().and_then(|list| Some((index, list))))
        .min_by_key(|(_, list)| list.val)?;

    let min_val = list_with_min_val.val;
    let next = list_with_min_val.clone().next;
    lists[index_of_list_with_min_val] = next;

    new_opt_box_list_node(min_val, _merge_k_lists_recursive(lists))
}
