pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by_key(|interval| (interval[1], -interval[0]));
    let (mut a, mut b) = (-1, -1);
    let mut ans = 0;
    for interval in intervals {
        if interval[0] > b {
            a = interval[1] - 1;
            b = interval[1];
            ans += 2;
        } else if interval[0] > a {
            a = b;
            b = interval[1];
            ans += 1;
        }
    }
    ans
}
