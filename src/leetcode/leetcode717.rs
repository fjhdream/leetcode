pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let n = bits.len();
    let mut idx = 0;
    while idx < n {
        if idx == n - 1 {
            return true;
        }
        idx += (bits[idx] + 1) as usize;
    }
    return false;
}

#[test]
fn test() {
    let bits = vec![1, 0, 0];
    let ans = is_one_bit_character(bits);
    assert_eq!(ans, true);
}

#[test]
fn test2() {
    let bits = vec![1, 1, 1, 0];
    let ans = is_one_bit_character(bits);
    assert_eq!(ans, false);
}

#[test]
fn test3() {
    let bits = vec![1, 0, 0, 0, 1, 0];
    let ans = is_one_bit_character(bits);
    assert_eq!(ans, false);
}

#[test]
fn test4() {
    let bits = vec![1, 1, 0];
    let ans = is_one_bit_character(bits);
    assert_eq!(ans, true);
}