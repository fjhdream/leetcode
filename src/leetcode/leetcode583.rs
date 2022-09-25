pub fn min_distance(word1: String, word2: String) -> i32 {
    let m = word1.len();
    let n = word2.len();
    let w1_bytes = word1.as_bytes();
    let w2_bytes = word2.as_bytes();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        let b1 = w1_bytes[i - 1];
        for j in 1..=n {
            let b2 = w2_bytes[j - 1];
            if b1 == b2 {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    let lcs = dp[m][n];
    (m - lcs + n - lcs) as i32
}

#[test]
pub fn test() {
    let str1 = String::from("sea");
    let str2 = String::from("eat");
    let ans = min_distance(str1, str2);
    assert_eq!(ans, 2);
}
