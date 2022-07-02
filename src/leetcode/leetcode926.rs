pub fn min_flips_mono_incr(s: String) -> i32 {
    let s_chars = s.chars().collect::<Vec<_>>();
    let n = s_chars.len();
    let mut left_ones = vec![0; n];
    let mut right_zeros = vec![0; n];
    for (i, &ch) in s_chars.iter().enumerate() {
        if i == 0 {
            left_ones[i] = if ch == '1' { 1 } else { 0 };
        } else {
            left_ones[i] = left_ones[i - 1] + if ch == '1' { 1 } else { 0 };
        }
    }

    for (i, &ch) in s_chars.iter().enumerate().rev() {
        if i == n - 1 {
            right_zeros[i] = if ch == '0' { 1 } else { 0 };
        } else {
            right_zeros[i] = right_zeros[i + 1] + if ch == '0' { 1 } else { 0 };
        }
    }

    let mut res = i32::MAX;
    for i in 1..n - 1 {
        res = res.min(left_ones[i - 1] + right_zeros[i + 1]);
    }
    res = res.min(left_ones[n - 1]).min(right_zeros[0]);
    res
}

pub fn min_flips_mono_incr2(s: String) -> i32 {
    let s_chars = s.chars().collect::<Vec<_>>();
    let n = s_chars.len();
    let mut dp_0 = if s_chars[0] == '0' { 0 } else { 1 };
    let mut dp_1 = if s_chars[0] == '1' { 0 } else { 1 };
    for i in 1..n {
        let pre_dp_0 = dp_0;
        dp_0 = dp_0 + if s_chars[i] == '0' { 0 } else { 1 };
        dp_1 = pre_dp_0.min(dp_1) + if s_chars[i] == '1' { 0 } else { 1 };
    }
    dp_0.min(dp_1)
}

#[test]
fn test() {
    let ans = min_flips_mono_incr("00110".to_string());
    assert_eq!(ans, 1);
    let ans = min_flips_mono_incr2("00110".to_string());
    assert_eq!(ans, 1);
}

#[test]
fn test2() {
    let ans = min_flips_mono_incr("010110".to_string());
    assert_eq!(ans, 2);
    let ans = min_flips_mono_incr2("010110".to_string());
    assert_eq!(ans, 2);
}

#[test]
fn test3() {
    let ans = min_flips_mono_incr("00011000".to_string());
    assert_eq!(ans, 2);
    let ans = min_flips_mono_incr2("00011000".to_string());
    assert_eq!(ans, 2);
}
