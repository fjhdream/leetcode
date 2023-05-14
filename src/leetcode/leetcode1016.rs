use std::collections::HashMap;

pub fn query_string(s: String, n: i32) -> bool {
    let mut map = HashMap::new();
    let s_chs = s.chars().collect::<Vec<_>>();
    let len = s_chs.len();
    for i in 0..len {
        if s_chs[i] == '0' {
            continue;
        }
        let mut num = 0;
        for j in i..len {
            num = num * 2 + s_chs[j] as i32 - '0' as i32;
            if num > n {
                break;
            }
            map.insert(num, true);
        }
    }
    map.len() == n as usize
}

#[test]
fn test() {
    let s = "0110".to_string();
    let n = 3;
    let res = true;
    assert_eq!(query_string(s, n), res);
    let s = "0110".to_string();
    let n = 4;
    let res = false;
    assert_eq!(query_string(s, n), res);
}
