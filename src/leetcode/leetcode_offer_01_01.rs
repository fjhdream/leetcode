pub fn is_unique(astr: String) -> bool {
    let mut set = 0;
    let u8_arr = astr.as_bytes();
    for &byte in u8_arr {
        let bit_num = (byte - 'a' as u8) as i32;
        if (set >> bit_num) & 1 == 1 {
            return false;
        } else {
            set |= 1 << bit_num;
        }
    }
    true
}

#[test]
fn test() {
    let astr = String::from("leetcode");
    let ans = is_unique(astr);
    assert_eq!(ans, false);
}

#[test]
fn test_1() {
    let astr = String::from("le");
    let ans = is_unique(astr);
    assert_eq!(ans, true);
}