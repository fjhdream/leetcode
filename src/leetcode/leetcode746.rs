pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let n = cost.len();
    let mut dp = vec![0; n + 1];
    for i in 2..=n {
        dp[i] = (dp[i - 2] + cost[i - 2]).min(cost[i - 1] + dp[i - 1]);
    }
    dp[n]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    }
    #[test]
    fn test_min_cost_climbing_stairs_2() {
        assert_eq!(
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
            6
        );
    }
}
