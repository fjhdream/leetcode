pub fn max_coins(nums: Vec<i32>) -> i32 {
    let mut simple_nums = [[1],[1]].join(nums.as_slice());
    let n = simple_nums.len();

    let mut dp = vec![vec![0;n];n];
    for i in (0..=n-1).rev() {
        for j in i+1..=n-1 {
            for k in i+1..j {
              dp[i][j] = dp[i][j].max(
                  dp[i][k] + dp[k][j] + simple_nums[i]*simple_nums[k]*simple_nums[j]
              );
            }
        }
    }
    dp[0][n-1]
}

#[test]
fn test_example() {
    let nums = vec![3,1,5,8];
    let ans = max_coins(nums.clone());
    assert_eq!(ans, 167);
}