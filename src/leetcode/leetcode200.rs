pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    let mut ans = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '1' {
                infect(&mut grid, x, y);
                ans += 1;
            }
        }
    }
    ans
}

fn infect(grid: &mut Vec<Vec<char>>, x: usize, y: usize) {
    if grid[x][y] == '0' {
        return;
    }
    if grid[x][y] == '2' {
        return;
    }
    grid[x][y] = '2';
    let dirs = vec![
        vec![0, 1], vec![1, 0],
        vec![-1, 0],vec![0, -1]
    ];
    for dir in dirs {
        let next_x = x as i32 + dir[0];
        let next_y = y as i32 + dir[1];
        if next_x < 0 || next_y < 0 || next_x >= grid.len() as i32 || next_y >= grid[0].len() as i32{
            continue;
        }
        infect(grid, next_x as usize, next_y as usize)
    }

}

#[test]
fn test() {
    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    let ans = num_islands(grid);
    assert_eq!(ans, 1);
}

#[test]
fn test_2() {
    let grid = vec![
        vec!['1', '1'],
    ];
    let ans = num_islands(grid);
    assert_eq!(ans, 1);
}


#[test]
fn test_3() {
    let grid = vec![
        vec!['1'],
        vec!['1']
    ];
    let ans = num_islands(grid);
    assert_eq!(ans, 1);
}