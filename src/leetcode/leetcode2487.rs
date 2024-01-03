use crate::leetcode::ListNode::ListNode;

pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack = vec![];

    while let Some(mut current_node) = head {
        head = current_node.next.take();
        stack.push(current_node);
    }

    let mut result: Option<Box<ListNode>> = None;

    while let Some(mut node) = stack.pop() {
        if result.is_none() || node.val >= result.as_ref().unwrap().val {
            node.next = result;
            result = Some(node);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(remove_nodes(None), None);
    }
}
