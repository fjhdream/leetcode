pub fn find_substring_in_wrapround_string(p: String) -> i32 {
    let p_chs = p.bytes().collect::<Vec<_>>();
    let mut cnt = [0; 26];
    let mut k = 0;
    for i in 0..p_chs.len() {
        if i > 0 && (p_chs[i] as i32 - p_chs[i-1] as i32 + 26) % 26 == 1 {
            k += 1; 
        } else {
            k = 1;
        }
        cnt[(p_chs[i] - 'a' as u8) as usize] = cnt[(p_chs[i] - 'a' as u8) as usize].max(k);
    }
    return cnt.iter().sum();
}

#[test]
fn test() {
    let ans = find_substring_in_wrapround_string("zab".to_string());
    assert_eq!(ans, 6);
}