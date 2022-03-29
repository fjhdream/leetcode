struct MaxStack {
    stack: Vec<i32>,
    max_stack: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxStack {

    fn new() -> Self {
        return MaxStack{
            stack: Vec::new(),
            max_stack: Vec::new()
        };
    }
    
    fn push(&mut self, x: i32) {
        if self.stack.is_empty() {
            self.stack.push(x);
            self.max_stack.push(x);
        } else {
            self.stack.push(x);
            let max_peak = *self.max_stack.last().unwrap(); 
            if max_peak > x {
                self.max_stack.push(max_peak);
            } else {
                self.max_stack.push(x);
            }
        }
    }
    
    fn pop(&mut self) -> i32 {
        self.max_stack.pop();
        self.stack.pop().unwrap()
    }
    
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    
    fn peek_max(&self) -> i32 {
        *self.max_stack.last().unwrap() 
    }
    
    fn pop_max(&mut self) -> i32 {
        let mut tmp = Vec::new();
        let max_num = self.peek_max();
        while self.top() != max_num{
            tmp.push(self.pop());
        }
        self.pop();
        while let Some(val) = tmp.pop() {
            self.push(val);
        }
        max_num
    }
}
