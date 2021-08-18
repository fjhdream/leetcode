pub fn check_record(n: i32) -> i32 {
    let n = n as usize;
    const MOD: i32 = 1000000007;
    let mut dp = vec![vec![vec![0;3];2];n+1];
    dp[0][0][0] = 1;
    for i in 1..=n {
        // 以P结尾
        for j in 0..2 {
            for k in 0..3 {
                dp[i][j][0] = (dp[i][j][0] + dp[i-1][j][k]) % MOD;
            }
        }

        //以A结尾
        for k in 0..3 {
            dp[i][1][0] = (dp[i][1][0] + dp[i-1][0][k]) % MOD;
        }

        //以L结尾
        for j in 0..2 {
            for k in 1..3 {
                dp[i][j][k] = (dp[i][j][k] + dp[i-1][j][k-1]) % MOD;
            }
        }
    }

    let mut sum = 0;
    for j in 0..2 {
        for k in 0..3 {
            sum = (sum + dp[n][j][k]) % MOD;
        }
    }
    return sum;
}

#[test]
fn test_example() {
    let n = 2;
    let ans = check_record(n);
    assert_eq!(ans, 8);
}