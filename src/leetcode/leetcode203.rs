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


#[allow(dead_code)]
pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut prev = &mut dummy;

    while let Some(node) = prev.next.take() {
        if node.val == val {
            prev.next = node.next;
        } else {
            prev.next = Some(node);
            prev = prev.next.as_mut().unwrap();
        }
    }
    dummy.next
}


