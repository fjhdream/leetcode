use std::collections::VecDeque;

pub fn find_the_winner(n: i32, k: i32) -> i32 {
    let mut queue = VecDeque::new();
    for i in 1..=n {
        queue.push_back(i);
    }
    while queue.len() > 1 {
        for _i in 0..k-1 {
            let val = queue.pop_front().unwrap();
            queue.push_back(val)
        }
        queue.pop_front();
    }
    queue.pop_front().unwrap()
}

// 约瑟夫环求解
pub fn find_the_winner2(n: i32, k: i32) -> i32 {
    let mut pos = 0;
    for i in 2..=n {
        pos = (pos + k) % i;
    }
    return pos + 1;
}