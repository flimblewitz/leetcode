fn main() {
    assert_eq!(
        Solution::add_two_numbers(
            new_list_node_from_vec(vec![2, 4, 3]),
            new_list_node_from_vec(vec![5, 6, 4])
        ),
        new_list_node_from_vec(vec![7, 0, 8])
    );
    assert_eq!(
        Solution::add_two_numbers(
            new_list_node_from_vec(vec![9, 9, 9, 9]),
            new_list_node_from_vec(vec![9, 9, 9, 9, 9, 9, 9])
        ),
        new_list_node_from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1])
    );
}

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

struct Solution;

// I wrote my own auxiliary constructor because the vanilla one sucks for the test cases
fn new_list_node_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    vec.into_iter()
        .rev()
        .fold(None, |acc, val| Some(Box::new(ListNode { val, next: acc })))
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1: Option<&Box<ListNode>> = l1.as_ref();
        let mut l2: Option<&Box<ListNode>> = l2.as_ref();

        let mut l3: Option<Box<ListNode>> = None;
        let mut current_l3_node = &mut l3;

        let mut carry = false;

        while l1.is_some() || l2.is_some() || carry {
            let mut val = 0;
            let mut carry_next = false;
            if let Some(inner_l1) = l1 {
                val += inner_l1.val;
                // the reason why we can't just use & is that the property "next" is an Option<Box<_>>, and we don't want to borrow the Option, we want to borrow the Box: l1 and l2 need to eventually each become a raw None
                l1 = inner_l1.next.as_ref();
            }
            if let Some(inner_l2) = l2 {
                val += inner_l2.val;
                l2 = inner_l2.next.as_ref();
            }

            if carry {
                val += 1
            }
            if val >= 10 {
                carry_next = true;
                val = val % 10;
            }

            *current_l3_node = Some(Box::new(ListNode::new(val)));
            // now we just need to shift the pointer to the the next node
            current_l3_node = &mut current_l3_node.as_mut().unwrap().next;

            carry = carry_next;
        }

        l3
    }

    // this one is based on a more optimal public solution. I found its logic for assigment and traversal unintuitive, but here it is with some educational comments
    pub fn alternative_2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1: Option<&Box<ListNode>> = l1.as_ref();
        let mut l2 = l2.as_ref();

        // this first node will be a dummy value that we'll exclude from the ultimate result
        let mut l3 = Some(Box::new(ListNode::new(0)));
        let mut current_l3_node = l3.as_mut();

        let mut carry = false;

        while l1.is_some() || l2.is_some() || carry {
            let mut val = 0;
            let mut carry_next = false;
            if let Some(inner_l1) = l1 {
                val += inner_l1.val;
                // the reason why we can't just use & is that the property "next" is an Option<Box<_>>, and we don't want to borrow the Option, we want to borrow the Box: l1 and l2 need to eventually each become a raw None
                l1 = inner_l1.next.as_ref();
            }
            if let Some(inner_l2) = l2 {
                val += inner_l2.val;
                l2 = inner_l2.next.as_ref();
            }

            if carry {
                val += 1
            }
            if val >= 10 {
                carry_next = true;
                val = val % 10;
            }

            // the reason why we can't just use current_l3_node.unwrap() to get the Box inside the Option is that unwrap CONSUMES the receiver, and we don't want to consume current_l3_node; we're still constructing the whole l3. In contrast, as_mut provides me a brand new Option which itself contains a mutable reference to the same Box inside. And since this a technically a brand new outer Option, we can unwrap THAT with impunity without consuming current_l3_node
            current_l3_node.as_mut().unwrap().next = Some(Box::new(ListNode { val, next: None }));
            // now that we've established the next node, it's time to shift our pointer to it for the subsequent iteration. The reason why we can just current_l3_node.unwrap() here is that we're reassigning it altogether, not just consuming it and leaving it destitute
            current_l3_node = current_l3_node.unwrap().next.as_mut();

            carry = carry_next;
        }

        l3.unwrap().next
    }

    // cloning is the easier option at first glance, but it's wasteful
    pub fn alternative_1(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut v: Vec<i32> = vec![];
        let mut carry = 0;
        let mut l1 = l1;
        let mut l2 = l2;

        loop {
            let mut next_carry = 0;
            let mut val = match (l1.clone(), l2.clone()) {
                (None, None) => {
                    if carry > 0 {
                        0
                    } else {
                        break;
                    }
                }
                (None, Some(inner_l2)) => {
                    let val = inner_l2.val;
                    l2 = inner_l2.next;
                    val
                }
                (Some(inner_l1), None) => {
                    let val = inner_l1.val;
                    l1 = inner_l1.next;
                    val
                }
                (Some(inner_l1), Some(inner_l2)) => {
                    let val = inner_l1.val + inner_l2.val;
                    l1 = inner_l1.next;
                    l2 = inner_l2.next;
                    val
                }
            };
            val += carry;
            if val >= 10 {
                next_carry = 1;
                val = val % 10;
            }
            v.push(val);
            carry = next_carry;
        }

        new_list_node_from_vec(v)
    }
}
