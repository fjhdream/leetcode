pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut dp = vec![vec![0; n]; n];
    for i in 0..n {
        dp[0][i] = matrix[0][i];
    }
    for i in 1..n {
        for j in 0..n {
            let mut min = dp[i - 1][j];
            if j > 0 {
                min = min.min(dp[i - 1][j - 1]);
            }
            if j < n - 1 {
                min = min.min(dp[i - 1][j + 1]);
            }
            dp[i][j] = min + matrix[i][j];
        }
    }
    dp[n - 1].iter().min().unwrap().to_owned()
}
