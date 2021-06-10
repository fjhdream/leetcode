#[allow(dead_code)]
pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let n = coins.len();
    let amount = amount as usize;
    let mut dp = vec![vec![0; amount + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        let coin = coins[i - 1];
        for j in 0..=amount {
            dp[i][j] = dp[i - 1][j];
            if (j as i32 - coin) >= 0 {
                dp[i][j] += dp[i][j - coin as usize];
            }
        }
    }
    dp[n][amount]
}

#[test]
fn test_example() {
    let amount = 5;
    let coins = vec![1, 2, 5];
    let ans = change(amount, coins);
    assert_eq!(ans, 4);
}
