use std::collections::VecDeque;

pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
    let n = patience.len();
    let mut adj = vec![Vec::new(); n];
    for edge in edges {
        adj[edge[0] as usize].push(edge[1] as usize);
        adj[edge[1] as usize].push(edge[0] as usize);
    }
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];
    queue.push_back(0);
    visited[0] = true;
    let mut ans = i32::MIN;
    let mut dist = 1;
    while !queue.is_empty() {
        let size = queue.len();
        for _ in 0..size {
            let node = queue.pop_front().unwrap();
            for &adj_node in adj[node].iter() {
                if visited[adj_node] {
                    continue;
                }
                let node_dist = patience[adj_node] * ((2 * dist - 1) / patience[adj_node]) + (2 * dist + 1);
                ans = ans.max(node_dist);
                queue.push_back(adj_node);
                visited[adj_node] = true;
            }
        }
       dist += 1;
    }
    return ans;
}

#[test]
fn test() {
    let ans = network_becomes_idle(vec![vec![0,1], vec![1,2]], vec![0,2,1]);
    assert_eq!(ans, 8);
}

#[test]
fn test2() {
    let ans = network_becomes_idle(vec![vec![0,1], vec![0,2], vec![1,2]], vec![0,10,10]);
    assert_eq!(ans, 3);
}

#[test]
fn test3() {
    let ans = network_becomes_idle(vec![vec![0,1], vec![0,2], vec![1,2]], vec![0,10,10]);
    assert_eq!(ans, 3);
}