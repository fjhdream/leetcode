use std::cell::RefCell;
use std::rc::Rc;

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

pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut sum = 0;
    sum_root_to_leaf_helper(&root, &mut sum, 0);
    sum
}

fn sum_root_to_leaf_helper(node: &Option<Rc<RefCell<TreeNode>>>, sum: &mut i32, curr_sum: i32) {
    if let Some(node) = node {
        let node = node.borrow();
        let curr_sum = curr_sum * 2 + node.val;
        if node.left.is_none() && node.right.is_none() {
            *sum += curr_sum;
        } else {
            sum_root_to_leaf_helper(&node.left, sum, curr_sum);
            sum_root_to_leaf_helper(&node.right, sum, curr_sum);
        }
    }
}
