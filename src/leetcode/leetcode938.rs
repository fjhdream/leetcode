use std::cell::RefCell;
use std::rc::Rc;
use super::treenode::TreeNode;

#[allow(dead_code)]
pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if root == None { return 0; }
    let rb = root.as_ref().unwrap().borrow_mut();
    let v = { if rb.val >= low && rb.val <= high  { rb.val } else { 0 } };
    match (rb.left.as_ref(), rb.right.as_ref()) {
        (Some(m), None) => { v + range_sum_bst(Some(m.clone()), low, high) },
        (None, Some(n)) => { v + range_sum_bst(Some(n.clone()), low, high) },
        (Some(m), Some(n)) => { v + range_sum_bst(Some(m.clone()), low, high) + range_sum_bst(Some(n.clone()), low, high)},
        _ => { v },
    }
}