use std::collections::VecDeque;

struct MovingAverage {
    queue: VecDeque<i32>,
    len: usize,
    sum: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {

    fn new(size: i32) -> Self {
        return MovingAverage {
            queue: VecDeque::new(),
            len: size as usize,
            sum: 0
        };
    }
    
    fn next(&mut self, val: i32) -> f64 {
        if self.queue.len() < self.len {
            self.queue.push_back(val);
            self.sum += val;
        } else {
            if let Some(pop_num) = self.queue.pop_front() {
                self.sum -= pop_num;
                self.sum += val;
                self.queue.push_back(val);
            }
        }
        return self.sum as f64 / self.queue.len() as f64;
    }
}