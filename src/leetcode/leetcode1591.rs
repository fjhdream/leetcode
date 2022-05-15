use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Matrix {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl Matrix {
    fn new() -> Self {
        Matrix { left: -1, right: -1, top: -1, bottom: -1 }
    }

    // 判断是否在矩阵内
    fn contains(&self, i: usize, j: usize) -> bool {
        self.top <= i as i32 && i as i32 <= self.bottom  && self.left <= j as i32 && j as i32 <= self.right
    }
}

pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
    let m = target_grid.len();
    let n = target_grid[0].len();

    let mut color_matrix = vec![Matrix::new(); 61];
    // 获取每种颜色的边界块
    for i in 0..m {
        for j in 0..n {
            let color = target_grid[i][j];
            let matrix = &mut color_matrix[color as usize];
            matrix.top = if matrix.top == -1 { i as i32} else { matrix.top.min(i as i32)};
            matrix.bottom = if matrix.bottom == -1 { i as i32} else { matrix.bottom.max(i as i32)};
            matrix.left = if matrix.left == -1 { j as i32} else { matrix.left.min(j as i32)};
            matrix.right = if matrix.right == -1 { j as i32} else { matrix.right.max(j as i32)};
        }
    }

    let mut is_connected = vec![vec![false; 61]; 61];
    let mut in_degree = vec![0; 61];
    let mut adj = vec![vec![]; 61];
    // 构图, 如果一个矩阵颜色u中有另一个颜色v, 说明u先于v涂色, u -> v  有边
    // 以当前格子颜色为v, 遍历所有颜色的matrix, 看是否有颜色为u的matrix包含当前格子
    for i in 0..m {
        for j in 0..n {
            let v = target_grid[i][j];
            for u in 1..=60 {
                let matrix = color_matrix[u as usize];
                    if u != v
                        && !is_connected[u as usize][v as usize]
                        && matrix.contains(i, j)
                    {
                        is_connected[u as usize][v as usize] = true;
                        adj[u as usize].push(v as usize);
                        in_degree[v as usize] += 1;
                    }
            
            }
        }
    }

    // bfs 查看是否有环
    let mut queue = VecDeque::<usize>::new();
    for color in 0..=60 {
        if in_degree[color] == 0 {
            queue.push_back(color);
        }
    }

    while !queue.is_empty() {
        let u = queue.pop_front().unwrap();
        for &v in adj[u].iter() {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }

   in_degree.iter().filter(|&&degree| degree != 0).count() == 0
}

#[test]
fn test() {
    let target_grid = vec![
        vec![6, 2, 2, 5],
        vec![2, 2, 2, 5],
        vec![2, 2, 2, 5],
        vec![4, 3, 3, 4],
    ];
    assert_eq!(is_printable(target_grid), true);
}

#[test]
fn test2() {
    let target_grid = vec![
        vec![1, 2, 1],
        vec![2, 1, 2],
        vec![1, 2, 1],
    ];
    assert_eq!(is_printable(target_grid), false);
}

#[test]
fn test3() {
    let target_grid = vec![
        vec![1, 1, 1, 1],
        vec![1, 2, 2, 1],
        vec![1, 2, 2, 1],
        vec![1, 1, 1, 1]
    ];
    assert_eq!(is_printable(target_grid), true);
}
