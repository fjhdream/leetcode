pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
    const INF: i32 = i32::MAX / 2;
    let mut g = vec![vec![INF; n as usize]; n as usize];
    for time in times {
        let (x, y) = (time[0] as usize - 1, time[1] as usize - 1);
        g[x][y] = time[2];
    }

    let mut dist = vec![INF; n as usize];
    dist[k as usize - 1] = 0;
    let mut used = vec![false; n as usize];

    for _ in 0..n {
        let mut x = usize::MAX;
        for y in 0..n as usize{
            if !used[y] && (x == usize::MAX || dist[y] < dist[x]) {
                x = y;
            }
        }

        used[x] = true;
        for y in 0..n as usize{
            dist[y] = dist[y].min(dist[x] + g[x][y]);
        }
    }

    let ans = *dist.iter().max().unwrap();
    if ans == INF { -1 } else { ans }
}

#[test]
pub fn test_example() {
    let times = vec![vec![2,1,1], vec![2,3,1], vec![3,4,1]];
    let n = 4;
    let k = 2;
    let ans = network_delay_time(times, n, k);
    assert_eq!(ans, 2);
}