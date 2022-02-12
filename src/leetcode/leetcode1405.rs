use std::collections::BinaryHeap;

    #[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Pair(i32, char);

fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut ans_chars = Vec::new();
    let mut heap = BinaryHeap::new();
    if a > 0 {heap.push(Pair(a, 'a'));} 
    if b > 0 {heap.push(Pair(b, 'b'));}
    if c > 0 {heap.push(Pair(c, 'c'));}
    while !heap.is_empty() {
        let mut cur_pair = heap.pop().unwrap();
        let n = ans_chars.len();
        if n >= 2 && ans_chars[n - 1] == cur_pair.1 && ans_chars[n - 2] == cur_pair.1 {
            if heap.is_empty() {
                break;
            }
            let mut next_pair = heap.pop().unwrap();
            ans_chars.push(next_pair.1);
            next_pair.0 -= 1;
            if next_pair.0 > 0 {
                heap.push(next_pair);
            }
            heap.push(cur_pair);
        } else {
            ans_chars.push(cur_pair.1);
            cur_pair.0 -= 1;
            if cur_pair.0 > 0 {
                heap.push(cur_pair);
            }
        }
    }

    ans_chars.iter().collect()
}

#[test]
fn test() {
    let ans = longest_diverse_string(7, 1, 0);
    assert_eq!(ans, "aabaa");
}
