pub fn num_ways(n: i32, relation: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut dp = vec![vec![0; n as usize]; k as usize + 1];
    dp[0][0] = 1;
    for i in 1..=k as usize {
        for pair in relation.iter() {
            let src = pair[0] as usize;
            let dst = pair[1] as usize;
            dp[i][dst] += dp[i - 1][src];
        }
    }

    dp[k as usize][n as usize - 1]
}

#[test]
fn test_example() {
    let n = 5;
    let relation = vec![
        vec![0, 2],
        vec![2, 1],
        vec![3, 4],
        vec![2, 3],
        vec![1, 4],
        vec![2, 0],
        vec![0, 4],
    ];
    let k = 3;
    let ans = num_ways(n, relation, k);
    assert_eq!(ans, 3);
}
