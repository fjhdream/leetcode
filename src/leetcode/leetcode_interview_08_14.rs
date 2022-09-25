pub fn count_eval(s: String, result: i32) -> i32 {
    let s_chars = s.chars().collect::<Vec<_>>();
    let n = s_chars.len();
    let mut dp = vec![vec![vec![-1;2];n];n];
    return recurse_eval(&s_chars, &mut dp, 0, n-1, result);
}   

fn recurse_eval(s_chars: &Vec<char>, dp: &mut Vec<Vec<Vec<i32>>>, left: usize, right: usize, result: i32) -> i32 {
    if left == right {
        return if s_chars[left].to_string().parse::<i32>().unwrap() == result { 1 } else { 0 };
    }

    if dp[left][right][result as usize] != -1 {
        return dp[left][right][result as usize];
    }

    let mut ans = 0;
    for mid in (left..right).step_by(2) {
        let op = s_chars[mid+1];
        for i in 0..=1 {
            for j in 0..=1 {
                if let Some(val) = operate(i, j, op) {
                    if val == result {
                        ans += recurse_eval(s_chars, dp, left, mid, i) * recurse_eval(s_chars, dp, mid+2, right, j);
                    }
                }
            }
        }
    }
    dp[left][right][result as usize] = ans;
    return ans;
}

fn operate(val1: i32, val2: i32, op: char) -> Option<i32> {
    return match op {
        '&' => Some(val1 & val2),
        '|' => Some(val1 | val2),
        '^' => Some(val1 ^ val2),
         _  => None,
    }
}

#[test]
fn test() {
    let s = String::from("1^0|0|1");
    let result = 0;
    let ans = count_eval(s, result);
    assert_eq!(ans, 2);
}