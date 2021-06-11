pub fn num_squares(n: i32) -> i32 {
    let mut answer = n;
    let n = n as usize;
    let mut dp = vec![answer; n + 1];
    dp[0] = 0;
    for i in 1..=n {
        let x = i * i;
        for j in x..=n {
            dp[j] = dp[j].min(dp[j - x] + 1);
        }
        answer = answer.min(dp[n])
    }
    answer
}