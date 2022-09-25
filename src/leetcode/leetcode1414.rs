pub fn find_min_fibonacci_numbers(mut k: i32) -> i32 {
    let mut fibs = vec![1, 1];
    let (mut a, mut b) = (1, 1);
    while a + b <= k {
        let c = a + b;
        fibs.push(a + b);
        a = b;
        b = c;
    }

    let mut ans = 0;
    let mut i = fibs.len() - 1;
    while k > 0 {
        let num = fibs[i];
        if k >= num {
            k -= num;
            ans += 1;
        }
        i -= 1;
    }
    ans
}

#[test]
fn test() {
    let ans = find_min_fibonacci_numbers(7);
    assert_eq!(ans, 2);
}

#[test]
fn test2() {
    let ans = find_min_fibonacci_numbers(19);
    assert_eq!(ans, 3);
}

#[test]
fn test3() {
    let ans = find_min_fibonacci_numbers(1);
    assert_eq!(ans, 1);
}
