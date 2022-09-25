use std::collections::VecDeque;

struct Node {
    node: usize,
    mask: usize,
    dist: usize,
}

pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut seen = vec![vec![false; 1 << n];n];
    for i in 0..n {
        queue.push_back(Node{ node: i, mask:1 << i, dist: 0});
        seen[i][1 << i] = true;
    }

    let mut ans = 0;
    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        if node.mask == (1 << n) - 1 {
            ans = node.dist as i32;
            break;
        }

        for v in &graph[node.node] {
            let mask_v = node.mask | (1 << *v as usize);
            if !seen[*v as usize][mask_v]{
                queue.push_back(Node{ node: *v as usize, mask: mask_v, dist: node.dist + 1});
                seen[*v as usize][mask_v] = true;
            }
        }
    }

    ans
}