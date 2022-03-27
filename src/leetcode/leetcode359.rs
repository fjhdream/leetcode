use std::collections::HashMap;

struct Logger {
    map: HashMap<String, i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Logger {

    fn new() -> Self {
        return Logger {
            map: HashMap::new()
        };
    }
    
    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if !self.map.contains_key(&message) {
            self.map.insert(message, timestamp);
            return true;
        } else {
            let stamp = self.map.get(&message).unwrap();
            if timestamp >= stamp + 10 {
                *self.map.entry(message).or_default() += timestamp;
                return true;
            } else {
                return false;
            }
        }
    }
}
