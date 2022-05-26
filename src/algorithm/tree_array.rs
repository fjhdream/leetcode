struct tree_array {
    trees: Vec<i32>,
}

impl tree_array {
    fn new(n: usize) -> Self {
        tree_array {
            trees: vec![0; n+1]
        }
    }
    
    fn low_bit(x: i32) -> i32 {
        x & -x
    }

    fn add(&mut self, x: usize, data: i32) {
        let mut i = x;
        while i < self.trees.len() {
            self.trees[i] += data;
            i += Self::low_bit(i as i32) as usize;
        }
    }

    fn query(&self, x: usize) -> i32 {
        let mut i = x;
        let mut res = 0;
        while i > 0 {
            res += self.trees[i];
            let diff = Self::low_bit(i as i32);
            if diff >= i as i32 {
                break;
            }
            i -= diff as usize;
        }
        res
    }

    // [left, right]
    fn range_query(&self, left: usize, right: usize) -> i32 {
        return self.query(right + 1) - self.query(left);
    }
}