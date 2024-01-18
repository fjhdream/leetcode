pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
    let mut ans = 0;
    let mut seen = [[false; 26]; 26];
    for s in words {
        let s = s.as_bytes();
        let x = (s[0] - b'a') as usize;
        let y = (s[1] - b'a') as usize;
        if seen[y][x] {
            ans += 1; // s 和 seen 中的 y+x 匹配
        } else {
            seen[x][y] = true;
        }
    }
    ans
}
