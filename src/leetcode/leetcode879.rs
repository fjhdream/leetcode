#[allow(dead_code)]
pub fn profitable_schemes(n: i32, min_profit: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
    let len = group.len();
    let mod_num:i32  = 10_i32.pow(9) + 7;
    let mut dp = vec![vec![vec![0; min_profit as usize + 1];n as usize + 1]; len + 1];
    dp[0][0][0] = 1;
    let n = n as usize;
    let min_profit =  min_profit as usize;
    for i in 1..=len {
        let members = group[i-1];
        let earn = profit[i-1];
        for j in 0..=n{
            for k in 0..=min_profit{
                if j < members as usize{
                    dp[i][j][k] = dp[i-1][j][k];
                } else {
                    dp[i][j][k] = (dp[i-1][j][k] + dp[i-1][j-members as usize][0.max(k as i32-earn) as usize]) % mod_num;
                }
            }
        }
    }

    let mut sum = 0;
    for m in 0..=n{
        sum = (sum + dp[len][m][min_profit]) % mod_num;
    }
    sum
}

#[test]
pub fn test_example() {
    let n = 100;
    let min_profit = 100;
    let group = vec![2,5,36,2,5,5,14,1,12,1,14,15,1,1,27,13,6,59,6,1,7,1,2,7,6,1,6,1,3,1,2,11,3,39,21,20,1,27,26,22,11,17,3,2,4,5,6,18,4,14,1,1,1,3,12,9,7,3,16,5,1,19,4,8,6,3,2,7,3,5,12,6,15,2,11,12,12,21,5,1,13,2,29,38,10,17,1,14,1,62,7,1,14,6,4,16,6,4,32,48];
    let profit = vec![21,4,9,12,5,8,8,5,14,18,43,24,3,0,20,9,0,24,4,0,0,7,3,13,6,5,19,6,3,14,9,5,5,6,4,7,20,2,13,0,1,19,4,0,11,9,6,15,15,7,1,25,17,4,4,3,43,46,82,15,12,4,1,8,24,3,15,3,6,3,0,8,10,8,10,1,21,13,10,28,11,27,17,1,13,10,11,4,36,26,4,2,2,2,10,0,11,5,22,6];
    let ans = profitable_schemes(n, min_profit, group, profit);
    println!("{}", ans);
    assert_eq!(ans, 7);
}