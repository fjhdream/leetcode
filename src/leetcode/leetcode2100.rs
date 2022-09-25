pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
    let n = security.len();
    if time == 0 {
        return (0..n as i32).collect::<Vec<_>>();
    }
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    for i in 1..n {
        if security[i] <= security[i - 1] {
            left[i] = left[i - 1] + 1;
        } else {
            left[i] = 0;
        }
    }
    for i in (0..n - 1).rev() {
        if security[i] <= security[i + 1] {
            right[i] = right[i + 1] + 1;
        } else {
            right[i] = 0;
        }
    }
    let mut res = vec![];
    for i in 0..n {
        if left[i] >= time && right[i] >= time {
            res.push(i as i32);
        }
    }
    res
}

#[test]
fn test() {
    let security = vec![5, 3, 3, 3, 5, 6, 2];
    let time = 2;
    let ans = good_days_to_rob_bank(security, time);
    assert_eq!(ans, vec![2, 3]);
}

#[test]
fn test2() {
    let security = vec![1, 1, 1, 1, 1];
    let time = 0;
    let ans = good_days_to_rob_bank(security, time);
    assert_eq!(ans, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test3() {
    let security = vec![1, 2, 3, 4, 5, 6];
    let time = 2;
    let ans = good_days_to_rob_bank(security, time);
    assert_eq!(ans, vec![]);
}
