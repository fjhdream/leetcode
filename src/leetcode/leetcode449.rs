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
use std::rc::Rc;
use std::cell::RefCell;
struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        return Codec{};
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return String::new();
        } else {
            let mut vals = vec![];
            Self::dfs_serialize(&root, &mut vals);
            vals.iter().map(|val| val.to_string()).collect::<Vec<String>>().join(",")
        }
    }

    fn dfs_serialize(root: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if root.is_none() {
            return;
        } 
        let root = root.as_ref().unwrap();
        let root = root.borrow();
        vals.push(root.val);
        Codec::dfs_serialize(&root.left, vals);
        Codec::dfs_serialize(&root.right, vals);
        return;
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        } else {
            let vals = data.split(",").map(|val| val.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            Self::dfs_deserialize(&vals, 0, vals.len() - 1)
        }
    }

    fn dfs_deserialize(datas: &Vec<i32>, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        } else {
            let root = Rc::new(RefCell::new(TreeNode::new(datas[left])));
            let mut split = left;
            let split_val = datas[left];
            while split <= right && datas[split] <= split_val {
                split += 1;
            }
            let left_node = Codec::dfs_deserialize(datas, left + 1, split - 1);
            let right_node = Codec::dfs_deserialize(datas, split, right);
            root.borrow_mut().left = left_node;
            root.borrow_mut().right = right_node;
            return Some(root);
        }
    }
}

#[test]
fn test() {
    let codec = Codec::new();
    let root = codec.deserialize("2,1,3".to_string());
    println!("{:?}", root)
}


#[test]
fn test2() {
    let codec = Codec::new();
    let root = codec.deserialize("2,1,3".to_string());
    let data = codec.serialize(root);
    println!("{:?}", data)
}