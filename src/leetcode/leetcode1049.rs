#[allow(dead_code)]
pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
    let sum: i32 = stones.iter().sum();
    let neg = (sum / 2) as usize;
    let n = stones.len();
    let mut dp = vec![vec![false; neg+1]; n+1];
    dp[0][0] = true;
    for i in 1..=n {
        let stone =stones[i-1] as usize;
        for j in 0..=neg {
            dp[i][j] = dp[i-1][j];
            if j >= stone {
                dp[i][j] = dp[i][j] || dp[i-1][j-stone];
            }
        }
    }

    let mut ans = 0;
    for i in (0..=neg).rev() {
        if dp[n][i] {
            ans = i as i32;
            break;
        }
    }

    return sum as i32 - 2 * ans;
}

#[test]
fn test_example() {
    let stones = vec![2,7,4,1,8,1];
    let ans = last_stone_weight_ii(stones);
    assert_eq!(1, ans);
}
