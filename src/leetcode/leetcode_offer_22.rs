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





pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = &head;
    let mut k = k as usize;
    while fast.is_some() && k > 0 {
        fast = &fast.as_ref().unwrap().next;
        k -= 1;
    }
    while fast.is_some() {
        fast = &fast.as_ref().unwrap().next;
        slow = &slow.as_ref().unwrap().next;
    }
    slow.clone()
}
