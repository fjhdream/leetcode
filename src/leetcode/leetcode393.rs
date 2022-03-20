pub fn valid_utf8(data: Vec<i32>) -> bool {
    let n = data.len();
    let mut i = 0;
    while i < n {
        let t = data[i];
        let mut j = 7usize;
        while ((t >> j) & 1) == 1 {
            j -= 1;
        }
        let cnt = 7 - j;
        if cnt == 1 || cnt > 4 {
            return false;
        }
        if i + cnt >= n + 1 {
            return false;
        }
        let mut k = i + 1;       
        while k < i + cnt as usize {
            if ((data[k] >> 7) & 1) == 1 && ((data[k] >> 6) & 1) == 0 {
                k += 1;
                continue;
            }
            return false;
        }
        if cnt == 0 {
            i += 1;
        } else {
            i += cnt as usize;
        }
    }
    return true;
}

#[test]
fn test() {
    let ans = valid_utf8(vec![197, 130, 1]);
    assert_eq!(ans, true);
}

#[test]
fn test2() {
    let ans = valid_utf8(vec![237]);
    assert_eq!(ans, false);
}

#[test]
fn test3() {
    let ans = valid_utf8(vec![10]);
    assert_eq!(ans, true);
}
