use std::rc::Rc;
use std::cell::RefCell;

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
      right: None
    }
  }
}

pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    let val = root.as_ref().unwrap().borrow().val;
    return dfs(&root, val);
}

fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> bool {
    if root.is_none() {
        return true;
    }
    let root = root.as_ref().unwrap();
    if root.borrow().val != val {
        return false;
    } else {
        return dfs(&root.borrow().left, val) && dfs(&root.borrow().right, val);
    }
}
