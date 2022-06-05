pub fn consecutive_numbers_sum(n: i32) -> i32 {
    let mut res = 0;
    let mut k = 1;
    while k * k < 2 * n {
        if (2 * n) % k != 0 {
            k += 1;
            continue;
        }
        if ((2 * n) / k - (k - 1)) % 2 == 0 {
            res += 1;
        }
        k += 1;
    }
    res
}

#[test]
fn test() {
    let ans = consecutive_numbers_sum(15);
    assert_eq!(ans, 4);
}
