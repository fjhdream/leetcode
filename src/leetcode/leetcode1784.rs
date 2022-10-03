 pub fn check_ones_segment(s: String) -> bool {
    let mut count = 0;
    let mut i = 0;
    let s_chs : Vec<_> = s.chars().collect();
    while i < s_chs.len() {
        if s_chs[i] == '0' {
            i += 1;
            continue;
        } else {
            while i < s_chs.len() && s_chs[i] == '1' {
                i += 1;
            }
            count += 1;
            if count > 1 {
                return false;
            }
        }        
    }
    return true;
}

#[test]
fn test() {
    let ans = check_ones_segment("1001".to_string());
    assert!(!ans);
}

#[test]
fn test2() {
    let ans = check_ones_segment("110".to_string());
    assert!(ans);

}

