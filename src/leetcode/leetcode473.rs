pub fn makesquare(matchsticks: Vec<i32>) -> bool {
    let mut matchsticks = matchsticks;
    let mut sum = matchsticks.iter().sum::<i32>();
    if sum % 4 != 0 {
        return false;
    }
    let len = sum / 4;
    let n = matchsticks.len();
    // 当前选取火柴状态的长度
    let mut dp = vec![-1; 1 << n];
    dp[0] = 0;
    for s in 1..(1 << n) {
        for stick in 0..n {
            if s & (1 << stick) == 0 {
                continue;
            }
            let s1 = s ^ (1 << stick);
            if dp[s1] >= 0 && dp[s1] + matchsticks[stick] <= len {
                dp[s] = (dp[s1] + matchsticks[stick]) % len;
                break;
            }
        }
    }
    dp[(1 << n) - 1] == 0
}

#[test]
fn test() {
    let ans = makesquare(vec![1, 1, 2, 2, 2]);
    assert_eq!(ans, true);
}

#[test]
fn test2() {
    let ans = makesquare(vec![3, 3, 3, 3, 4]);
    assert_eq!(ans, false);
}
