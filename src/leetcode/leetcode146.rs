mod double_linked_list {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub type Link<T: Clone> = Option<Rc<RefCell<Node<T>>>>;

    pub struct List<T: Clone> {
        pub head: Link<T>,
        pub tail: Link<T>,
    }


    pub struct Node<T: Clone> {
        pub elem: T,
        pub next: Link<T>,
        pub prev: Link<T>,
    }

    impl<T: Clone> Node<T> {
        pub fn new(elem: T) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Node {
                elem: elem,
                next: None,
                prev: None,
            }))
        }
    }

    impl<T: Clone> List<T> {
        pub fn new() -> Self {
            List {
                head: None,
                tail: None,
            }
        }

        pub fn push_front(&mut self, elem: T) {
            let new_head = Node::new(elem);

            match self.head.take() {
                Some(old_head) => {
                    old_head.borrow_mut().prev = Some(new_head.clone()); // new_head rc+1 2(new_head, old_head.prev)
                    new_head.borrow_mut().next = Some(old_head); // move so old_head new alias is new_head.next
                    self.head = Some(new_head); // -1 old_head // new_head new alias is self.head
                }
                None => {
                    self.tail = Some(new_head.clone()); // rc = 2
                    self.head = Some(new_head); // new_head ref not existed, move to self.head rc = 2
                }
            }
        }

        pub fn push_tail(&mut self, elem: T) -> Link<T> {
            let new_tail = Node::new(elem);

            match self.tail.take() {
                Some(old_tail) => {
                    old_tail.borrow_mut().next = Some(new_tail.clone());
                    new_tail.borrow_mut().prev = Some(old_tail);
                    self.tail = Some(new_tail.clone());
                }
                None => {
                    self.tail = Some(new_tail.clone()); // 先 clone 否则 move 后无法使用
                    self.head = Some(new_tail.clone());
                }
            }
            Some(new_tail)
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|old_head| {
                match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev.take(); // -1 old
                        self.head = Some(new_head); // +1 new head
                    }
                    None => {
                        self.tail.take(); // -1 old
                    }
                }
                // Rc<RefCell<Node>> 解Rc try_unwrap 解RefCell into_inner
                // Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
                old_head.borrow().elem.clone()
            })
        }


        pub fn pop_tail(&mut self) -> Option<T> {
            self.tail.take().map(|old_tail| {
                match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next.take();
                        self.tail = Some(new_tail);
                    }
                    None => {
                        self.head.take(); // -1 old
                        // the self.tail has been None
                    }
                }
                // Rc<RefCell<Node>> 解Rc try_unwrap 解RefCell into_inner
                // Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
                old_tail.borrow().elem.clone()
            })
        }

        pub fn remove(&mut self, node: Link<T>) {
            if let Some(node_rc) = node {
                /*
                let node_ref= node_rc.borrow();
                let f_h = node_ref.prev.is_none();
                let f_t = node_ref.next.is_none();
                std::mem::drop(node_ref); // it is not awesome
                */
                let f_h = node_rc.borrow().prev.is_none(); // the node_ref is temp and drop!
                let f_t = node_rc.borrow().next.is_none(); // the same

                match (f_h, f_t) {
                    (false, false) => {
                        let prev_node = node_rc.borrow_mut().prev.take();
                        let next_node = node_rc.borrow_mut().next.take();
                        match (prev_node, next_node) {
                            (Some(prev_node), Some(next_node)) => {
                                // Rc<RefCell<Link<T>>
                                // prev_node.next = next_node
                                // next_node.prev = prev_node

                                // drop the node
                                prev_node.
                                    borrow_mut().
                                    next.
                                    take();
                                next_node.
                                    borrow_mut().
                                    prev.
                                    take();

                                prev_node.
                                    borrow_mut().
                                    next =
                                    Some(next_node.clone());
                                next_node.
                                    borrow_mut().
                                    prev =
                                    Some(prev_node.clone());
                            }
                            (_, _) => { panic!("logic error") }
                        }
                    }
                    (true, _) => {
                        self.pop_front();
                    }
                    (_, true) => {
                        self.pop_tail();
                    }
                }
            }
        }
    }

    impl<T: Clone> Drop for List<T> {
        fn drop(&mut self) {
            while self.pop_front().is_some() {}
        }
    }

    #[cfg(test)]
    mod test {
        use super::List;

        #[test]
        fn basics() {
            let mut list = List::new();

    // Check empty list behaves right
            assert_eq!(list.pop_front(), None);

    // Populate list
            list.push_front(1);
            list.push_front(2);
            list.push_front(3);

    // Check normal removal
            assert_eq!(list.pop_front(), Some(3));
            assert_eq!(list.pop_front(), Some(2));

    // Push some more just to make sure nothing's corrupted
            list.push_front(4);
            list.push_front(5);

    // Check normal removal
            assert_eq!(list.pop_front(), Some(5));
            assert_eq!(list.pop_front(), Some(4));

    // Check exhaustion
            assert_eq!(list.pop_front(), Some(1));
            assert_eq!(list.pop_front(), None);
        }

        #[test]
        fn enhance() {
            let mut list = List::new();

    // Check empty list behaves right
            assert_eq!(list.pop_front(), None);
            assert_eq!(list.pop_tail(), None);

    // Populate list
            list.push_front(1);
            list.push_front(3);
            list.push_front(5);
            list.push_tail(2);

            // 5 3 1 2

    // Check normal removal
            assert_eq!(list.pop_front(), Some(5));
            assert_eq!(list.pop_front(), Some(3));
            assert_eq!(list.pop_tail(), Some(2));


    // Push some more just to make sure nothing's corrupted
            list.push_front(7);
            list.push_tail(4);
            list.push_tail(6);
            list.push_front(9);

            // 9 7 1 4 6

    // Check normal removal
            assert_eq!(list.pop_front(), Some(9));
            assert_eq!(list.pop_front(), Some(7));
            assert_eq!(list.pop_tail(), Some(6));
            assert_eq!(list.pop_tail(), Some(4));

    // Check exhaustion
            assert_eq!(list.pop_tail(), Some(1));
            assert_eq!(list.pop_tail(), None);
        }

        #[test]
        fn test_remove() {
            let mut list = List::new();

    // Check empty list behaves right
            assert_eq!(list.pop_front(), None);
            assert_eq!(list.pop_tail(), None);

    // Populate list
            list.push_front(1);
            list.push_front(3);
            list.push_front(5);
            let node = list.push_tail(2);

            // 5 3 1 2

    // Check normal removal
            list.push_tail(4);
            list.push_tail(6);

            // 5 3 1 2 4 6

            list.remove(node);
            assert_eq!(list.pop_tail(), Some(6));
            assert_eq!(list.pop_tail(), Some(4));
            assert_eq!(list.pop_tail(), Some(1));

            assert_eq!(list.pop_front(), Some(5));
            assert_eq!(list.pop_front(), Some(3));
            assert_eq!(list.pop_front(), None);
        }
    }
}

use std::collections::HashMap;
use self::double_linked_list::{Link, List};

type Key = i32;
type KV = (Key, i32);

type Node = Link<KV>;

struct LRUCache {
    // map: map to Node, queue for priority
    map: HashMap<Key, Node>,
    queue: List<KV>,
    cap: i32,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            map: HashMap::new(),
            queue: List::new(),
            cap: capacity,
        }
    }

    fn push2back(&mut self,  node: Node) {
        let node_copy = node.clone();
        if let Some(mut node_rc) = node {
            let elem: KV = node_rc.borrow().elem;
            self.queue.remove(node_copy);
            let new_node = self.queue.push_tail(elem);
            self.map.insert(elem.0, new_node);
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.map.get(&key) {
            if let Some(node_rc) = node {
                let v = node_rc.borrow().elem.1;
                // remove will free then give elem,so no more rc on it!
                self.push2back(Some(node_rc.to_owned()));
                return v;
            }
            return -1;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        // in Cache
        if let Some(node) = self.map.get_mut(&key) {
            let node_clone = node.clone();
            if let Some(node_rc) = node {
                node_rc.borrow_mut().elem.1 = value;
                self.push2back(node_clone);
            }
        } else {
            // push to cache
            let len = self.map.len();
            if len < self.cap as usize {
                let node = self.queue.push_tail((key, value));
                self.map.insert(key, node);
            } else {
                // Full: pop and push
                let pop_node = self.queue.pop_front();
                if let Some(pop_node) = pop_node{
                    self.map.remove(&pop_node.0);

                    let node = self.queue.push_tail((key, value));
                    // self.map[&key] = (value, len - 1); // the IndexMut is not implemented for custom type
                    self.map.insert(key, node);
                } else {
                    panic!("logic error: ensure the queue is not empty!");
                }
            }
        }
    }

//作者：llouice
//链接：https://leetcode-cn.com/problems/lru-cache/solution/rust-shi-yong-optionrcrefcellt-xie-liao-yi-tian-by/
//来源：力扣（LeetCode）
//著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
}