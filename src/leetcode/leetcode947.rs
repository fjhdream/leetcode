use std::collections::HashMap;
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let mut union_find = UnionFind::new();
    for stone in stones.iter() {
        union_find.union(stone[0] + 100000, stone[1]);
    }
    stones.len() as i32 - union_find.count
}
struct UnionFind {
    parent : HashMap<i32, i32>,
    count : i32,
}

impl UnionFind {
    fn new() -> UnionFind {
        let parent = HashMap::new();
        let count = 0;
        UnionFind {
            parent,
            count
        }
    }

    fn find(&mut self, x : i32) -> i32 {
        if !self.parent.contains_key(&x) {
            self.parent.insert(x, x);
            self.count += 1;
        }

        if &x != self.parent.get(&x).unwrap() {
            let v = self.find( *self.parent.get(&x).unwrap());
            self.parent.insert(x, v);
        }
        
        *self.parent.get(&x).unwrap()
    }

    fn union(&mut self, x : i32, y : i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        self.parent.insert(root_x, root_y);
        self.count -= 1;
    }
}
