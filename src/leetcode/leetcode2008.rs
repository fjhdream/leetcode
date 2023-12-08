use std::collections::HashMap;

pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
    let mut dp = vec![0_i64; n as usize + 1];
    let mut ride_map = HashMap::new();
    for ride in rides {
        let (start, end, tip) = (ride[0] as usize, ride[1] as usize, ride[2] as i64);
        ride_map.entry(end).or_insert(vec![]).push(ride);
    }
    for i in 1..=n as usize {
        dp[i] = dp[i - 1];
        for ride in ride_map.get(&i).unwrap_or(&vec![]) {
            dp[i] = dp[i].max(dp[ride[0] as usize] + (ride[1] - ride[0] + ride[2]) as i64);
        }
    }
    dp[n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let rides = vec![vec![2, 5, 4], vec![1, 5, 1]];
        assert_eq!(max_taxi_earnings(5, rides), 7);
    }
}
