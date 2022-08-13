struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(list1), Some(list2)) => {
                if list1.val >= list2.val {
                    return Some(Box::new(ListNode {
                        val: list2.val,
                        next: Solution::merge_two_lists(Some(list1), list2.next)
                    }))
                }
                Some(Box::new(ListNode {
                    val: list1.val,
                    next: Solution::merge_two_lists(list1.next, Some(list2))
                }))
            }
        }
    }
}

fn main() {
    let mut l1 = Some(Box::new(ListNode::new(1)));
    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    l1.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut l2 = Some(Box::new(ListNode::new(1)));
    l2.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    l2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let ret = Solution::merge_two_lists(l1, l2);

    let mut expect = Some(Box::new(ListNode::new(1)));
    expect.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
    expect.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    expect.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    expect.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    expect.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    assert_eq!(ret, expect);
}