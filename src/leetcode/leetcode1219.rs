pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();
    let mut ans = i32::MIN;
    for x in 0..m {
        for y in 0..n {
            if grid[x][y] == 0 {
                continue;
            }
            ans = ans.max(dfs(x, y, &mut grid));
        }
    }
    ans
}

fn dfs(x: usize, y: usize, grid: &mut Vec<Vec<i32>>) -> i32 {
    let dirs = vec![vec![-1, 0], vec![1, 0], vec![0, 1], vec![0, -1]];
    let m =grid.len();
    let n = grid[0].len();
    let rec = grid[x][y];
    let mut ans = rec;
    grid[x][y] = 0;
    for dir in dirs {
        let nx = x as i32 + dir[0];
        let ny = y as i32 + dir[1];
        if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 && grid[nx as usize][ny as usize] > 0 {
            ans = ans.max(dfs(nx as usize, ny as usize, grid) + rec);
        }
    }
    grid[x][y] = rec;
    ans
}


#[test]
fn test() {
    let grid = vec![
        vec![0,6,0],
        vec![5,8,7],
        vec![0,9,0]
    ];
    let ans = get_maximum_gold(grid);
    assert_eq!(ans, 24);
}

#[test]
fn test2() {
    let grid = vec![
        vec![1,0,7],
        vec![2,0,6],
        vec![3,4,5],
        vec![0,3,0],
        vec![9,0,20]
    ];
    let ans = get_maximum_gold(grid);
    assert_eq!(ans, 28);
}
