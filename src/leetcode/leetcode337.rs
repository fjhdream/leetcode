use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::treenode::TreeNode;

struct RobInfo {
    rob: i32,
    not_rob: i32,
}

pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }
    let ans = rob_helper(root);
    ans.not_rob.max(ans.rob)
}

fn rob_helper(root: Option<Rc<RefCell<TreeNode>>>) -> RobInfo {
    if root.is_none() {
        return RobInfo { rob: 0, not_rob: 0 };
    }
    let rob_lef = rob_helper(root.as_ref().unwrap().borrow().left.clone());
    let rob_rig = rob_helper(root.as_ref().unwrap().borrow().right.clone());
    let stolen = rob_lef.not_rob + rob_rig.not_rob + root.as_ref().unwrap().borrow().val;
    let not_stolen = (rob_lef.rob + rob_rig.rob)
        .max(rob_lef.not_rob + rob_rig.not_rob)
        .max(rob_lef.rob + rob_rig.not_rob)
        .max(rob_lef.not_rob + rob_rig.rob);
    return RobInfo {
        rob: stolen,
        not_rob: not_stolen,
    };
}
