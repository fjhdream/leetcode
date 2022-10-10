use std::collections::VecDeque;

pub fn score_of_parentheses(s: String) -> i32 {
    let mut deque = VecDeque::new();
    deque.push_back(0);
    for ch in s.chars() {
        if ch == '(' {
            deque.push_back(0);
        } else {
            let v = deque.pop_back().unwrap();
            let top = deque.pop_back().unwrap() + 1.max(2 * v);
            deque.push_back(top);
        }
    }
    deque.pop_back().unwrap()
}

#[test]
fn test() {
    let ans = score_of_parentheses("(()(()))".to_string());
    assert_eq!(ans, 6);
}
