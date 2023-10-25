pub fn punishment_number(n: i32) -> i32 {
    let mut ans = 0;
    for i in 1..=n {
        if check(i * i, i) {
            ans += i * i;
        }
    }
    ans
}

fn check(t: i32, x: i32) -> bool {
    if t == x {
        return true;
    }
    let mut d = 10;
    while t >= d && t % d <= x {
        if check(t / d, x - (t % d)) {
            return true;
        }
        d *= 10;
    }
    false
}
