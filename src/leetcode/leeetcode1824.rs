pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
    let inf = 0x3fffffff;
    let mut dp = vec![vec![inf; 3]; obstacles.len()];
    dp[0] = vec![1, 0, 1];
    let n = obstacles.len();
    for i in 1..n {
        for j in 0..3_usize {
            if j as i32 == obstacles[i] - 1 {
                dp[i][j] = inf;
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
        let min = *dp[i].iter().min().unwrap();
        for j in 0..3_usize {
            if j as i32 == obstacles[i] - 1 {
                continue;
            } else {
                dp[i][j] = dp[i][j].min(min + 1);
            }
        }
    }
    *dp[n - 1].iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_side_jumps() {
        assert_eq!(min_side_jumps(vec![0, 1, 2, 3, 0]), 2);
        assert_eq!(min_side_jumps(vec![0, 1, 1, 3, 3, 0]), 0);
        assert_eq!(min_side_jumps(vec![0, 2, 1, 0, 3, 0]), 2);
    }
}
