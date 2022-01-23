use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};

struct StockPrice {
    max_time_stamp: i32,
    time_price_map: HashMap<i32, i32>,
    pg_max : BinaryHeap<(i32, i32)>,
    pg_min : BinaryHeap<Reverse<(i32, i32)>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {

    fn new() -> Self {
        return StockPrice {
            max_time_stamp: 0,
            time_price_map: HashMap::new(),
            pg_max: BinaryHeap::new(),
            pg_min: BinaryHeap::new(),
        }
    }
    
    fn update(&mut self, timestamp: i32, price: i32) {
        self.max_time_stamp = self.max_time_stamp.max(timestamp);
        self.time_price_map.insert(timestamp, price);
        self.pg_max.push((price, timestamp));
        self.pg_min.push(Reverse((price, timestamp)));
    }
    
    fn current(&self) -> i32 {
        *self.time_price_map.get(&self.max_time_stamp).unwrap()
    }
    
    fn maximum(&mut self) -> i32 {
        loop {
            let(price, timestamp) = self.pg_max.peek().unwrap();
            if self.time_price_map.get(timestamp).unwrap() == price {
                return *price;
            }
            self.pg_max.pop();
        }
    }
    
    fn minimum(&mut self) -> i32 {
        loop {
            let Reverse((price, timestamp)) = self.pg_min.peek().unwrap();
            if self.time_price_map.get(timestamp).unwrap() == price {
                return *price;
            }
            self.pg_min.pop();
        }
    }
}
