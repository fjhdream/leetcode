use std::collections::VecDeque;

pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
    let m = forest.len();
    let n = forest[0].len();
    let mut trees = Vec::new();
    for i in 0..m {
        for j in 0..n {
            if forest[i][j] > 1 {
                trees.push((forest[i][j],i as i32, j as i32));
            }
        }
    }
    trees.sort();

    let mut start = (0, 0);
    let mut res = 0;
    for tree in trees {
        let step = bfs(&forest, start, (tree.1, tree.2));
        if step == -1 {
            return -1;
        } else {
            res += step;
            start = (tree.1, tree.2);
        }
    }
    res
}

fn bfs(forest: &Vec<Vec<i32>>, start: (i32, i32), end: (i32, i32)) -> i32 {
    if start == end {
        return 0;
    }
    let mut step = 0;
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; forest[0].len()]; forest.len()];
    visited[start.0 as usize][start.1 as usize] = true;
    queue.push_back(start);
    while !queue.is_empty() {
        step += 1;
        let size = queue.len();
        for _ in 0..size {
            let cur = queue.pop_front().unwrap();
            for i in 0..4 {
                let next = match i {
                    0 => (cur.0 + 1, cur.1),
                    1 => (cur.0 - 1, cur.1),
                    2 => (cur.0, cur.1 + 1),
                    3 => (cur.0, cur.1 - 1),
                    _ => unreachable!(),
                };
                if next.0 < 0
                    || next.0 >= forest.len() as i32
                    || next.1 < 0
                    || next.1 >= forest[0].len() as i32
                {
                    continue;
                }
                if !visited[next.0 as usize][next.1 as usize]
                    && forest[next.0 as usize][next.1 as usize] != 0
                {
                    if next == end {
                        return step;
                    }
                    queue.push_back(next);
                    visited[next.0 as usize][next.1 as usize] = true;
                }
            }
        }
    }
    -1
}

#[test]
fn test() {
    let ans = cut_off_tree(vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]]);
    assert_eq!(ans, 6);
}
