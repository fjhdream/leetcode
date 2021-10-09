pub fn find_min_moves(machines: Vec<i32>) -> i32 {
    let n = machines.len();
    let sum = machines.iter().sum::<i32>();
    if sum % n as i32 != 0 {
        return -1;
    }
    let avg = sum / n as i32;
    let mut left_sum = 0;
    let mut right_sum = sum;
    let mut ans = 0;
    for i in 0..n {
        right_sum -= machines[i];
        let a = 0.max(avg * i as i32 - left_sum);
        let b = 0.max(avg * (n - i - 1) as i32 - right_sum);
        ans = ans.max(a + b);
        left_sum += machines[i];
    }
    ans as i32
}

#[test]
pub fn test() {
    let machines = vec![1, 0, 5];
    let ans = find_min_moves(machines);
    assert_eq!(ans, 3);
}

#[test]
pub fn test2() {
    let machines = vec![0, 3, 0];
    let ans = find_min_moves(machines);
    assert_eq!(ans, 2);
}