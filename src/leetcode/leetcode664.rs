pub fn strange_printer(s: String) -> i32 {
    let bytes = s.as_bytes();
    let n = bytes.len();
    let mut dp = vec![vec![0; n]; n];
    for i in (0..n).rev() {
        dp[i][i] = 1;
        for j in i+1..n {
            let si = bytes[i];
            let sj = bytes[j];
            if si == sj {
                dp[i][j] = dp[i][j-1];
            } else {
                let mut min_value = i32::MAX;
                for k in i..j {
                    min_value = min_value.min(dp[i][k] + dp[k+1][j]);
                }
                dp[i][j] = min_value;
            }
        }
    }

    dp[0][n-1]
}