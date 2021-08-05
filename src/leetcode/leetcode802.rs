pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::VecDeque;
    let n = graph.len();
    let mut in_degree = vec![0; n];
    let mut rev_graph = Vec::new();

    for _i in 0..n {
        rev_graph.push(Vec::<usize>::new());
    }

    for i in 0..n {
        for j in &graph[i] {
            rev_graph[*j as usize].push(i);
        }
        in_degree[i] = graph[i].len();
    }

    let mut queue = VecDeque::new();
    for (i, degree) in in_degree.iter().enumerate() {
        if *degree == 0 { 
            queue.push_back(i);
        }
    }

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        for x in &rev_graph[node] {
            in_degree[*x] -= 1;
            if in_degree[*x] == 0 {
                queue.push_back(*x);
            }
        }
    }

    in_degree.iter().enumerate().filter_map(|(i, x)| if *x == 0 { Some(i as i32) } else { return None }).collect()
}

#[test]
pub fn test() {
    let graph = vec![vec![1,2],vec![2,3],vec![5],vec![0],vec![5],vec![],vec![]];
    let ans = eventual_safe_nodes(graph);
    println!("{:?}", ans);
}