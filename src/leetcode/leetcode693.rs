pub fn has_alternating_bits(mut n: i32) -> bool {
    let mut pre = -1;
    while n > 0 {
        if pre != n & 1 {
            pre = n & 1;
            n >>= 1;
        } else {
            return false;
        }
    }
    return true;
}

#[test]
fn test() {
    let ans = has_alternating_bits(5);
    assert!(ans);
}

#[test]
fn test2() {
    let ans = has_alternating_bits(7);
    assert!(!ans);
}

#[test]
fn test3() {
    let ans = has_alternating_bits(i32::MAX);
    assert!(!ans);
}
