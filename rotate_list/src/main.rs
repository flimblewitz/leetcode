fn main() {
    todo!("linked lists are still hard");
    println!(
        "{:?}",
        Solution::rotate_right(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None }))
                        }))
                    }))
                }))
            })),
            2
        )
    );
}

struct Solution {}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
        // if head.is_none() {
        //     return head;
        // }

        // so I guess I want to set k = k % length
        // and then I find the kth item and seize its next "new_head", replacing it with None
        // I then take that new_head, find its last item, and set its next to 1
        let cloned_head = head.clone();
        let mut current = &head;
        let mut num_nodes = 0;
        while let Some(node) = current {
            num_nodes += 1;
            current = &node.next;
        }

        k = k % num_nodes;
        if k == 0 {
            return head;
        }

        let mut current = &mut head;
        for _ in 1..(num_nodes - k) {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                panic!("oops");
            }
        }
        if let Some(new_tail) = current {
            if let Some(mut new_head) = new_tail.next.clone() {
                // I need to find the last node and set its next to the original head
                let mut last = &mut new_head;
                while let Some(_) = &mut last.next {
                    // if there is a next node, change our reference to it and continue
                    // if there is no next node, set its next to head and break
                    match &mut last.next {
                        Some(another_node) => last = another_node,
                        None => {
                            // last.next = head;
                            break;
                        }
                    };
                }
                // last.next = head.clone();
                last.next = cloned_head;

                // then I can set head to new_head and set new_tail.next to None
                new_tail.next = None;
                head = Some(new_head);
            } else {
                panic!("triple oops");
            }
        } else {
            panic!("double oops");
        }

        head
    }
}
