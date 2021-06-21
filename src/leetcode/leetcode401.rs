pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let mut ans = Vec::new();
    for i in 0..1024 as i32 {
        let h = i >> 6;
        let m = i & 63;
        if h < 12 && m < 60 && i.count_ones() == turned_on as u32 {
            ans.push(h.to_string() + ":" + if m < 10 { "0" } else { "" } + &m.to_string());
        }
    }
    ans
}

#[test]
fn test_example() {
    let turned_on = 6;
    let ans = read_binary_watch(turned_on);
    println!("{:?}", ans);
}