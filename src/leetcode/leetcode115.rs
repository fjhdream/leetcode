pub fn num_distinct(s: String, t: String) -> i32 {
    let (m, n) = (s.len(), t.len());
    if m < n { return 0;}
    let mut dp = vec![vec![0; n+1]; m+1];
    for i in 0..=m {
        dp[i][n] = 1;
    }

    let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());
    for i in (0..=m-1).rev() {
        for j in (0..=n-1).rev() {
            if s_bytes[i] == t_bytes[j] {
                dp[i][j] = dp[i+1][j+1] + dp[i+1][j];
            } else {
                dp[i][j] = dp[i+1][j];
            }
        }
    }

    dp[0][0]
}

#[test]
pub fn test_example() {
    let ans = num_distinct("".to_string(), "s".to_string());
    assert_eq!(0, ans)
}