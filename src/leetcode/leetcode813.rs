pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
    let len = nums.len();
    let mut prefix = vec![0_f64; len + 1];
    for i in 0..len {
        prefix[i + 1] = prefix[i] + nums[i] as f64;
    }
    let mut dp = vec![vec![0_f64; k as usize + 1]; len + 1];
    for i in 1..=len {
        dp[i][1] = prefix[i] / i as f64;
    }
    for j in 2..=k as usize {
        for i in j..=len {
            for x in (j - 1)..i {
                let avg = (prefix[i] - prefix[x]) / (i - x) as f64;
                dp[i][j] = dp[i][j].max(dp[x][j - 1] + avg)
            }
        }
    }
    dp[len][k as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert(x: f64, y: f64) -> bool {
        return (x - y).abs() < 1e-6;
    }

    #[test]
    fn test1() {
        let ans = largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3);
        assert!(assert(ans, 20.0f64))
    }

    #[test]
    fn test2() {
        let ans = largest_sum_of_averages(vec![1, 2, 3, 4, 5, 6, 7], 4);
        assert!(assert(ans, 20.50000f64))
    }
}
