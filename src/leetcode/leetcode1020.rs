pub fn num_enclaves(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let (m, n) = (grid.len(), grid[0].len());
    for y in 0..n {
        if grid[0][y] == 1 {
            reset_land(0, y, &mut grid)
        }
        if grid[m - 1][y] == 1 {
            reset_land(m - 1, y, &mut grid);
        }
    }
    for x in 0..m {
        if grid[x][0] == 1 {
            reset_land(x, 0, &mut grid)
        }
        if grid[x][n - 1] == 1 {
            reset_land(x, n - 1, &mut grid);
        }
    }
    grid.iter().for_each(|row| {
        row.iter().for_each(|val| {
            if *val == 1 {
                ans += 1
            }
        })
    });
    ans
}

fn reset_land(x: usize, y: usize, grid: &mut Vec<Vec<i32>>) {
    let dirs = vec![vec![0, 1], vec![0, -1], vec![1, 0], vec![-1, 0]];
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    grid[x][y] = 0;
    for dir in dirs {
        let (next_x, next_y) = (x as i32 + dir[0], y as i32 + dir[1]);
        if next_x >= 0
            && next_x < m
            && next_y >= 0
            && next_y < n
            && grid[next_x as usize][next_y as usize] == 1
        {
            reset_land(next_x as usize, next_y as usize, grid);
        }
    }
}

#[test]
fn test() {
    let grid = vec![
        vec![0, 0, 0, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 1, 0],
        vec![0, 0, 0, 0],
    ];
    let ans = num_enclaves(grid);
    assert_eq!(ans, 3);
}

#[test]
fn test2() {
    let grid = vec![vec![1]];
    let ans = num_enclaves(grid);
    assert_eq!(ans, 0);
}

#[test]
fn test3() {
    let grid = vec![
        vec![0, 1, 1, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 1, 0],
        vec![0, 0, 0, 0],
    ];
    let ans = num_enclaves(grid);
    assert_eq!(ans, 0);
}
