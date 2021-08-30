use rand::{thread_rng, Rng};
struct Solution {
    pre: Vec<i32>,
    total: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(w: Vec<i32>) -> Self {
        let n = w.len();
        let mut pre = vec![0; n];
        pre[0] = w[0];
        for i in 1..n {
            pre[i] = pre[i-1] + w[i];
        }
        let total = w.iter().sum();
        Solution {
            pre: pre,
            total: total,
        }

    }
    
    fn pick_index(&self) -> i32 {
        let x = thread_rng().gen_range(1..self.total + 1);
        match self.pre.binary_search(&x) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}