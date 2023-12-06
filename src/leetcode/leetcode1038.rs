use std::{cell::RefCell, rc::Rc};

use super::treenode::TreeNode;
pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, s: &mut i32) {
        if let Some(x) = node {
            let mut x = x.borrow_mut();
            dfs(x.right.as_ref(), s);
            *s += x.val;
            x.val = *s;
            dfs(x.left.as_ref(), s)
        }
    }
    let mut s = 0;
    dfs(root.as_ref(), &mut s);
    root
}
