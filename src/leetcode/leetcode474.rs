#[allow(dead_code)]
pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let length = strs.len();
    let mut dp = vec![vec![vec![0; n as usize+1]; m as usize +1]; length+1];
    for i in 1..=length {
        let(zeros, ones) = get_zeros_ones(strs[i-1].as_str());
        for j in 0..=m as usize {
            for k in 0..=n as usize {
                dp[i][j][k] = dp[i-1][j][k];
                if j >= zeros && k >= ones {
                    dp[i][j][k] = dp[i][j][k].max(dp[i-1][j-zeros][k-ones] + 1);
                }
            }
        }
    }
    dp[length][m as usize][n as usize]
}

#[allow(dead_code)]
fn get_zeros_ones(str : &str) -> (usize, usize) {
    let (mut zeros, mut ones) = (0, 0);
    for i in str.chars() {
        if i == '0' {
            zeros += 1;
        } else if i == '1' {
            ones += 1;
        }
    }
    return (zeros, ones);
}

#[test]
fn test_example() {
    let strs = vec!["10".to_string(), "0001".to_string(), "111001".to_string(), "1".to_string(), "0".to_string()];
    let m = 5;
    let n = 3;
    let ans = find_max_form(strs, m, n);
    assert_eq!(ans, 4);
}