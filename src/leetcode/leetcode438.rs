pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let s_bytes = s.as_bytes();
    let p_bytes = p.as_bytes();
    let s_len = s_bytes.len();
    let p_len = p_bytes.len();

    if s_len < p_len {
        return Vec::new();
    }

    let mut s_count = [0;26];
    let mut p_count = [0;26];
    for i in 0..p_len {
        s_count[(s_bytes[i] - 'a' as u8) as usize] += 1;
        p_count[(p_bytes[i] - 'a' as u8) as usize] += 1;
    }

    let mut ans = Vec::new();
    if s_count.eq(&p_count) {
        ans.push(0);
    }

    for i in 0..(s_len - p_len) {
        s_count[(s_bytes[i] - 'a' as u8) as usize] -= 1;
        s_count[(s_bytes[i + p_len] - 'a' as u8) as usize] += 1;
        if s_count.eq(&p_count) {
            ans.push(i as i32 + 1);
        }
    }

    ans
}

#[test]
fn test() {
    let s = String::from("cbaebabacd");
    let p = String::from("abc");
    let ans = find_anagrams(s, p);
    assert_eq!(ans, vec![0, 6]);
}