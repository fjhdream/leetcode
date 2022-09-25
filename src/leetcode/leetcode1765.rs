use std::{collections::VecDeque, vec};

pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = is_water.len();
    let n = is_water[0].len();
    let mut ret = vec![vec![-1; n]; m];

    let mut queue = VecDeque::new();
    is_water.iter().enumerate().for_each(|(row, row_val)| {
        row_val.iter().enumerate().filter(|(_, val)| {**val == 1})
        .for_each(|(col, _)| {queue.push_back((row, col)); ret[row][col] = 0;});
    });
    
    let dirs = vec![vec![0,1], vec![0,-1], vec![1,0], vec![-1,0]];
    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        for dir in &dirs{
            let next_row = row as i32 + dir[0];
            let next_col = col as i32 + dir[1];
            if next_row < 0 || next_col < 0 || next_row >= m as i32 || next_col >= n as i32 {
                continue;
            } 
            if  ret[next_row as usize][next_col as usize] == -1 {
                ret[next_row as usize][next_col as usize] = ret[row][col] + 1;
                queue.push_back((next_row as usize, next_col as usize));
            }
        }
    }

    ret
}

#[test]
fn test() {
    let is_water = vec![vec![0,1], vec![0,0]];
    let ans = vec![vec![1,0], vec![2,1]];
    let expect_ans = highest_peak(is_water);
    assert_eq!(ans, expect_ans);
}

#[test]
fn test2() {
    let is_water = vec![vec![0,0,1], vec![1,0,0], vec![0,0,0]];
    let ans = vec![vec![1,1,0], vec![0,1,1], vec![1,2,2]];
    let expect_ans = highest_peak(is_water);
    assert_eq!(ans, expect_ans);
}