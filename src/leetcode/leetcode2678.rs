pub fn count_seniors(details: Vec<String>) -> i32 {
    let mut ans = 0;
    for detail in details {
        let old: &i32 = &detail[11..13].parse().unwrap();
        if *old > 60 {
            ans += 1;
        }
    }
    ans
}
