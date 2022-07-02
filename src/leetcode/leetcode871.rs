use std::collections::BinaryHeap;

pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    let n = stations.len();
    let mut dp = vec![0; n + 1];
    dp[0] = start_fuel;
    for i in 0..n {
        for j in (0..=i).rev() {
            if dp[j] >= stations[i][0] {
                dp[j + 1] = dp[j + 1].max(dp[j] + stations[i][1]);
            }
        }
    }
    for (i, &dist) in dp.iter().enumerate() {
        if dist >= target {
            return i as i32;
        }
    }
    return -1;
}

pub fn min_refuel_stops2(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
    let mut heap = BinaryHeap::new();
    let (mut ans, mut prev, mut fuel) = (0, 0, start_fuel);
    let n = stations.len();
    for i in 0..=n {
        let curr = if i < n { stations[i][0] } else { target };
        fuel -= curr - prev;
        while fuel < 0 && !heap.is_empty() {
            fuel += heap.pop().unwrap();
            ans += 1;
        }
        if fuel < 0 {
            return -1;
        }

        if i < n {
            heap.push(stations[i][1]);
            prev = curr;
        }
    }
    return ans;
}

#[test]
fn test() {
    let ans = min_refuel_stops(100, 1, vec![vec![10, 100]]);
    assert_eq!(ans, -1);
}
