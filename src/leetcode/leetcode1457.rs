use std::cell::RefCell;
use std::rc::Rc;

use crate::leetcode::treenode::TreeNode;

pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, p: &mut [i32; 10]) -> i32 {
        if let Some(x) = node {
            let x = x.borrow();
            p[x.val as usize] ^= 1; // 修改 x.val 出现次数的奇偶性
            let res;
            if x.left.is_none() && x.right.is_none() {
                // x 是叶子节点
                res = if p.iter().sum::<i32>() <= 1 { 1 } else { 0 };
            } else {
                res = dfs(x.left.as_ref(), p) + dfs(x.right.as_ref(), p);
            };
            // 恢复到递归 x 之前的状态（不做这一步就把 x.val 算到其它路径中了）
            p[x.val as usize] ^= 1;
            return res;
        }
        0
    }
    let mut p = [0; 10];
    dfs(root.as_ref(), &mut p)
}
