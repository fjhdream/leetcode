pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    const INF: i32 = 10000 * 101 + 1;
    let mut f = vec![vec![INF; n as usize]; k as usize + 2];
    f[0][src as usize] = 0;
    for t in 1..= (k + 1) as usize {
        for flight in &flights {
            let j = flight[0] as usize;
            let i = flight[1] as usize;
            let cost = flight[2];
            f[t][i] = f[t][i].min(f[t - 1][j] + cost);
        }
    }
    let mut ans = INF;
    for t in 1..=(k + 1) as usize {
        ans = ans.min(f[t][dst as usize]);
    }

    return if ans == INF { -1 } else { ans };
}

#[test]
fn test_example() {
    let n = 3;
    let flights = vec![vec![0,1,100],vec![1,2,100],vec![0,2,500]];
    let src = 0;
    let dst = 2;
    let k = 1;
    let ans = find_cheapest_price(n,flights, src, dst, k);
    assert_eq!(ans, 200);
}