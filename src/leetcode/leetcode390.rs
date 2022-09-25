pub fn last_remaining(n: i32) -> i32 {
    let mut a1 = 1;
    let (mut k, mut cnt, mut step) = (0, n, 1);
    while cnt > 1 {
        if k % 2 == 0 {
            a1 = a1 + step;
        } else {
            a1 = if cnt % 2 == 0 {
                a1
            } else {
                a1 + step
            }
        }
        k += 1;
        cnt = cnt >> 1;
        step = step << 1;
    }
    a1
}