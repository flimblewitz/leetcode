fn main() {
    todo!("linked lists in rust are hard after all")
    // let v = vec![1, 2, 3, 4, 5];
    // let mut head = Some(Box::new(ListNode::new(v[0])));
    // let mut current_node_ref = &mut head;
    // for val in v.iter().skip(1) {
    //     current_node_ref.unwrap().as_ref().as_ref().next = Some(Box::new(ListNode::new(*val)));
    //     current_node_ref = &mut current_node_ref.unwrap().as_ref().as_ref().next;
    // }
    // println!("{:?}", Solution::remove_nth_from_end(head, 2));
}

// // Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

// struct Solution {}
// impl Solution {
//     pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
//         let n = n as usize;
//         if n == 1 {
//             return None;
//         }

//         // let mut node = head.clone();
//         let mut nodes = vec![];
//         // let mut node_ref = &mut head;
//         let mut node = head.clone();
//         while let Some(ref boxed_node) = node {
//             nodes.push(Some(boxed_node.clone()));
//             node = boxed_node.next.clone();
//         }

//         let num_nodes = nodes.len();

//         let prev = nodes[num_nodes - n - 1].clone().unwrap();
//         let to_delete = nodes[num_nodes - n].clone().unwrap();
//         nodes[num_nodes - n - 1] = Some(Box::new(ListNode {
//             val: prev.val,
//             next: to_delete.next,
//         }));
//         nodes.remove(nodes.len() - n);
//         nodes[0].clone()
//     }
// }
