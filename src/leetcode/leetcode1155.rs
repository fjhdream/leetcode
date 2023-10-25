pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    const MOD: i32 = 1000000007;
    let mut dp = vec![vec![0; target as usize + 1]; n as usize + 1];
    dp[0][0] = 1;
    for i in 1..=n as usize {
        for j in 1..=target {
            for x in 1..=k {
                if j - x >= 0 {
                    dp[i][j as usize] = (dp[i][j as usize] + dp[i - 1][(j - x) as usize]) % MOD;
                }
            }
        }
    }
    dp[n as usize][target as usize]
}

#[test]
fn test() {
    let ans = num_rolls_to_target(30, 30, 500);
    assert_eq!(ans, 222616187);
}
