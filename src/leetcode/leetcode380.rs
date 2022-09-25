use std::collections::HashMap;
use rand::Rng;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    vec: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.map.insert(val, self.vec.len());
        self.vec.push(val);
        true
    }
    
    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            return false;
        }
        let index = self.map[&val];
        let last = self.vec.len() - 1;
        self.vec[index] = self.vec[last];
        self.map.insert(self.vec[last], index);
        self.vec.pop();
        self.map.remove(&val);
        true
    }
    
    fn get_random(&self) -> i32 {
        let index = rand::thread_rng().gen_range(0..self.vec.len());
        self.vec[index]
    }
}
