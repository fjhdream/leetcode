use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    key: i32,
    value: i32,
    freq: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            freq: 1,
            prev: None,
            next: None,
        }))
    }
}

struct LFUCache {
    capacity: usize,
    min_freq: i32,
    key_to_node: HashMap<i32, Rc<RefCell<Node>>>,
    freq_to_dummy: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            min_freq: 0,
            key_to_node: HashMap::new(),
            freq_to_dummy: HashMap::new(),
        }
    }

    fn get_node(&mut self, key: i32) -> Option<Rc<RefCell<Node>>> {
        if let Some(node) = self.key_to_node.get(&key) {
            // 有这本书
            let node = node.clone();
            Self::remove(node.clone()); // 把这本书抽出来
            let freq = node.borrow().freq;
            let dummy = self.freq_to_dummy.get(&freq).unwrap();
            if Rc::ptr_eq(dummy, dummy.borrow().prev.as_ref().unwrap()) {
                // 抽出来后，这摞书是空的
                self.freq_to_dummy.remove(&freq); // 移除空链表
                if self.min_freq == freq {
                    // 这摞书是最左边的
                    self.min_freq += 1;
                }
            }
            node.borrow_mut().freq += 1; // 看书次数 +1
            self.push_front(freq + 1, node.clone()); // 放在右边这摞书的最上面
            return Some(node);
        }
        None
    }

    fn new_list() -> Rc<RefCell<Node>> {
        let dummy = Node::new(0, 0);
        dummy.borrow_mut().prev = Some(dummy.clone());
        dummy.borrow_mut().next = Some(dummy.clone());
        dummy
    }

    fn push_front(&mut self, freq: i32, x: Rc<RefCell<Node>>) {
        let dummy = self
            .freq_to_dummy
            .entry(freq)
            .or_insert_with(|| Self::new_list());
        let next = dummy.borrow().next.clone();
        x.borrow_mut().prev = Some(dummy.clone());
        x.borrow_mut().next = next.clone();
        dummy.borrow_mut().next = Some(x.clone());
        next.unwrap().borrow_mut().prev = Some(x.clone());
    }

    fn remove(x: Rc<RefCell<Node>>) {
        let prev = x.borrow().prev.clone().unwrap();
        let next = x.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.get_node(key) {
            return node.borrow().value;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.get_node(key) {
            node.borrow_mut().value = value;
            return;
        }
        if self.key_to_node.len() == self.capacity {
            let dummy = self.freq_to_dummy.get(&self.min_freq).unwrap();
            let back_node = dummy.borrow().prev.clone().unwrap();
            let key = back_node.borrow().key;
            self.key_to_node.remove(&key);
            Self::remove(back_node);
            if Rc::ptr_eq(dummy, dummy.borrow().prev.as_ref().unwrap()) {
                self.freq_to_dummy.remove(&self.min_freq);
            }
        }
        let node = Node::new(key, value);
        self.key_to_node.insert(key, node.clone());
        self.push_front(1, node.clone());
        self.min_freq = 1;
    }
}
