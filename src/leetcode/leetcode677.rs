use std::collections::HashMap;

struct MapSum {
    map: HashMap<u64, i32>,
    prefix_map: HashMap<u64, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {
    fn new() -> Self {
        MapSum {
            map: HashMap::new(),
            prefix_map: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        let hash_key = hash(key.clone());
        let map_val = if self.map.contains_key(&hash_key) {
            *self.map.get(&hash_key).unwrap()
        } else {
            0
        };
        let delta = val - map_val;
        self.map.insert(hash_key, val);
        for i in 1..=key.len() {
            *self.prefix_map.entry(hash(&key[..i])).or_insert(0) += delta;
        }
    }

    fn sum(&self, prefix: String) -> i32 {
        let hash_key = hash(prefix);
        return if self.prefix_map.contains_key(&hash_key) {
            self.prefix_map.get(&hash_key).unwrap().to_owned()
        } else {
            0
        };
    }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
fn hash<T: Hash>(t: T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn test_example() {
    let mut map_sum = MapSum::new();
    map_sum.insert(String::from("apple"), 3);
    let ans1 = map_sum.sum(String::from("ap"));
    assert_eq!(ans1, 3);
    map_sum.insert(String::from("ap"), 2);
    let ans2 = map_sum.sum(String::from("ap"));
    assert_eq!(ans2, 5);
}
