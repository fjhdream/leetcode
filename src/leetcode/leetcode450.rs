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
use std::cell::RefCell;
use std::rc::Rc;
pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let mut root = root.unwrap();
    if root.borrow().val > key {
        let t = delete_node(root.borrow().left.clone(), key);
        root.borrow_mut().left = t;
    } else if root.borrow().val < key {
        let t = delete_node(root.borrow().right.clone(), key);
        root.borrow_mut().right = t;
    } else {
        if root.borrow().left.is_none() {
            return root.borrow().right.clone();
        }
        if root.borrow().right.is_none() {
            return root.borrow().left.clone();
        }
        let mut successor = find_successor(root.borrow().right.clone());
        let successor_val = successor.unwrap().borrow().val;
        root.borrow_mut().val = successor_val;
        let t = delete_node(root.borrow().right.clone(), successor_val);
        root.borrow_mut().right = t;
    }

    Some(root)
}

fn find_successor(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return None;
    }
    let mut root = root.unwrap();
    while root.borrow().left.is_some() {
        let t = root.borrow().left.clone();
        root = t.unwrap();
    }
    Some(root)
}
