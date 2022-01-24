use std::{cmp::Reverse, collections::VecDeque, vec};

pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
    let mut graph = vec![Vec::new(); n as usize + 1];
    for edge in edges {
        graph[edge[0] as usize].push(edge[1] as usize);
        graph[edge[1] as usize].push(edge[0] as usize);
    }
    let mut first_shortest_path = vec![i32::MAX; n as usize + 1];
    let mut second_shortest_path = vec![i32::MAX; n as usize + 1];
    first_shortest_path[1] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(Reverse((1, 0)));
    while second_shortest_path[n as usize] == i32::MAX {
        let Reverse((cur, len)) = queue.pop_front().unwrap();
        for &next in &graph[cur] {
            if len + 1 < first_shortest_path[next] {
                first_shortest_path[next] = len + 1;
                queue.push_back(Reverse((next, len + 1)));
            } else if len + 1 > first_shortest_path[next] && len + 1 < second_shortest_path[next] {
                second_shortest_path[next] = len + 1;
                queue.push_back(Reverse((next, len + 1)));
            }
        }
    }
    let mut ret = 0;
    for _ in 0..second_shortest_path[n as usize] {
        if ret % (2 * change) >= change {
            ret = ret + (2 * change - ret % (2 * change));
        }
        ret = ret + time;
    }
    ret
}

#[test]
fn test() {
    let n = 5;
    let edges = vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]];
    let time = 3;
    let change = 5;
    let ans = second_minimum(n, edges, time, change);
    assert_eq!(ans, 13);
}

#[test]
fn test2() {
    let n = 6;
    let edges = vec![
        vec![1, 2],
        vec![1, 3],
        vec![2, 4],
        vec![3, 5],
        vec![5, 4],
        vec![4, 6],
    ];
    let time = 3;
    let change = 100;
    let ans = second_minimum(n, edges, time, change);
    assert_eq!(ans, 12);
}
