use std::{cell::RefCell, rc::Rc};

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

struct CBTInserter {
    candinate: Vec<Option<Rc<RefCell<TreeNode>>>>,
}

impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut candinate = Vec::new();
        candinate.push(root);
        let mut i = 0;
        while i < candinate.len() {
            let (left, right) = (
                candinate[i].as_ref().unwrap().borrow().left.clone(),
                candinate[i].as_ref().unwrap().borrow().right.clone(),
            );
            if left.is_some() {
                candinate.push(left);
            }
            if right.is_some() {
                candinate.push(right);
            }
            i += 1;
        }
        CBTInserter { candinate }
    }

    fn insert(&mut self, val: i32) -> i32 {
        let pos = self.candinate.len();
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        self.candinate.push(Some(node.clone()));
        if (pos & 1) == 1 {
            self.candinate[(pos - 1) / 2]
                .as_ref()
                .unwrap()
                .borrow_mut()
                .left = Some(node.clone())
        } else {
            self.candinate[(pos - 1) / 2]
                .as_ref()
                .unwrap()
                .borrow_mut()
                .right = Some(node.clone())
        }
        self.candinate[(pos - 1) / 2].as_ref().unwrap().borrow().val
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.candinate[0].clone()
    }
}
