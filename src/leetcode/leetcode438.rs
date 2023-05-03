pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let s_bytes = s.as_bytes();
    let p_bytes = p.as_bytes();
    let s_len = s.len();
    let p_len = p.len();
    let mut ans = vec![];
    if s_len < p_len {
        return ans;
    }
    let mut pattern = vec![0; 26];
    for i in 0..p_len {
        pattern[(p_bytes[i] - b'a') as usize] += 1;
    }

    let mut start = 0;
    let mut end = 0;
    let mut window = vec![0; 26];
    while end < s_len {
        let idx = (s_bytes[end] - b'a') as usize;
        window[idx] += 1;
        if end - start + 1 == p_len {
            if window == pattern {
                ans.push(start as i32);
            }
            window[(s_bytes[start] - b'a') as usize] -= 1;
            start += 1;
        }
        end += 1;        
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