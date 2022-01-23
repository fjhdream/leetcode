use std::collections::{HashMap, VecDeque};

pub fn min_jumps(arr: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    for (idx, &val) in arr.iter().enumerate() {
        let steps = map.entry(val).or_insert(Vec::new());
        steps.push(idx);
    }
    let mut visited = vec![false; arr.len()];

    let mut queue = VecDeque::new();
    queue.push_back((0usize, 0));
    visited[0] = true;
    let last_idx = arr.len() - 1;
    while !queue.is_empty() {
        let (idx, mut step) = queue.pop_front().unwrap();
        if idx == last_idx {
            return step;
        }
        step += 1;
        if map.contains_key(&arr[idx]) {
            for &next_idx in map.get(&arr[idx]).unwrap() {
                visited[next_idx] = true;
                queue.push_back((next_idx, step));
                
            }
            map.remove(&arr[idx]);
        }
        if idx as i32 -1 >= 0 && !visited[idx - 1] {
            visited[idx - 1] = true;
            queue.push_back((idx - 1, step));
        }
        let post_idx = idx + 1;
        if  post_idx < arr.len() && !visited[idx + 1] {
            visited[idx + 1] = true;
            queue.push_back((post_idx, step));
        }
    }
    -1
}

#[test]
fn test() {
    let arr = vec![100,-23,-23,404,100,23,23,23,3,404];
    let ans = min_jumps(arr);
    assert_eq!(ans, 3);
}


#[test]
fn test1() {
    let arr = vec![100];
    let ans = min_jumps(arr);
    assert_eq!(ans, 0);
}

#[test]
fn test2() {
    let arr = vec![7,6,9,6,9,6,9,7];
    let ans = 1;
    assert_eq!(ans, 1);
}