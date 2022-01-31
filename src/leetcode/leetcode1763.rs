pub fn longest_nice_substring(s: String) -> String {
    let s_chars = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut max_len = 0;
    let mut max_pos = 0;
    for i in 0..n {
        let mut lower = 0;
        let mut upper = 0;
        for j in i..n {
            if s_chars[j].is_lowercase() {
                lower |= 1 << (s_chars[j] as i32 - 'a' as i32);
            } else {
                upper |= 1 << (s_chars[j] as i32 - 'A' as i32);
            }
            if upper == lower && j - i + 1 > max_len {
                max_pos = i;
                max_len = j - i + 1;
            }
        }
    }
    s[max_pos..max_pos + max_len].to_owned()
}

#[test]
fn test() {
    let s = String::from("YazaAay");
    let ans = longest_nice_substring(s);
    assert_eq!(ans, "aAa");
}

#[test]
fn test2() {
    let s = String::from("Bb");
    let ans = longest_nice_substring(s);
    assert_eq!(ans, "Bb");
}

#[test]
fn test3() {
    let s = String::from("c");
    let ans = longest_nice_substring(s);
    assert_eq!(ans, "");
}
