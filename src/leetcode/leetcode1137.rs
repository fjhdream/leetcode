pub fn tribonacci(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2{
        return 1;
    }

    let (mut p, mut q, mut r, mut s) = (0, 0, 1, 1);
    for _ in 3..=n {
        p = q;
        q = r;
        r = s;
        s = p + q + r;
    }
    s
}

#[test]
fn test_example() {
    let n = 25;
    let ans = tribonacci(n);
    assert_eq!(ans, 1389537);
}