use rand::prelude::*;

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

    struct Solution {
        node: Option<Box<ListNode>>,
        rand: ThreadRng
    }

    impl Solution {

        fn new(head: Option<Box<ListNode>>) -> Self {
            Solution{node: head, rand: thread_rng()}
        }
        
        fn get_random(&mut self) -> i32 {
            let mut i = 1;
            let mut ans = 0;
            let mut cur = self.node.as_ref();
            while cur.is_some() {
                if self.rand.gen_range(0..=i) == 0 {
                    ans = cur.unwrap().val;
                }
                i += 1;
                cur = cur.unwrap().next.as_ref();
            }

            ans
        }
    }