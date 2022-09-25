pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
    let mut min_a = m;
    let mut min_b = n;
    for op in ops {
        min_a = min_a.min(op[0]);
        min_b = min_b.min(op[1]);
    }
    min_a * min_b
}

#[test]
pub fn test_example() {
    let m = 3;
    let n = 3;
    let ops = vec![vec![2, 2], vec![3, 3]];
    let ans = max_count(m, n, ops);
    assert_eq!(ans, 4);
}
