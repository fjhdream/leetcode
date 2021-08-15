use std::vec;

pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    let mod_num = 10_i32.pow(9) + 7;
    let mov = max_move as usize;
    let m = m as usize;
    let n = n as usize;
    let start_row = start_row as usize;
    let start_column = start_column as usize;

    let mut ans = 0;
    let directions: Vec<Vec<i32>> = vec![vec![-1,0], vec![1,0], vec![0,1], vec![0, -1]];
    let mut dp = vec![vec![vec![0;n];m];mov + 1];
    dp[0][start_row][start_column] = 1;
    for i in 0..mov {
        for j in 0..m {
            for k in 0..n {
                let count = dp[i][j][k];
                if count > 0 {
                    for dir in &directions {
                        let j1 = j as i32 + dir[0];
                        let k1 = k as i32 + dir[1];
                        if j1 >= 0 && j1 < m as i32 && k1 >= 0 && k1 < n as i32 {
                            let j1 = j1 as usize;
                            let k1 = k1 as usize;
                            dp[i+1][j1][k1] = (dp[i+1][j1][k1] + count) % mod_num;
                        } else{
                            ans = (ans + count) % mod_num;
                        }
                    }
                }
            }
        }
    }
    ans
}

#[test]
fn test_example() {
    let (m, n, max_move, start_row, start_column) = (2, 2, 2, 0, 0);
    let ans = find_paths(m, n, max_move, start_row, start_column);
    assert_eq!(ans, 6);
}

#[test]
fn test_example2() {
    let (m, n, max_move, start_row, start_column) = (1, 3, 3, 0, 1);
    let ans = find_paths(m, n, max_move, start_row, start_column);
    assert_eq!(ans, 12);
}