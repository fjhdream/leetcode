

struct MyQueue {
    data: Vec<i32>,
    bake: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyQueue {
            data: Vec::new(),
            bake: Vec::new()
        }
    }
    
    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.data.push(x);
    }
    
    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        if self.bake.is_empty() {
            while let Some(v) = self.data.pop() {
                self.bake.push(v)
            }
        }
        self.bake.pop().unwrap()
    }
    
    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        if self.bake.is_empty() {
            while let Some(v) = self.data.pop() {
                self.bake.push(v)
            }
        }
        *self.bake.last().unwrap()
    }
    
    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.bake.is_empty() && self.data.is_empty() 
    }
}